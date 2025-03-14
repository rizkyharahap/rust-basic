use serde::Serialize;
use validator::{Validate, ValidateArgs};

pub mod validatorapp {
    pub mod validator {
        use std::borrow::Cow;

        use validator::ValidationError;

        use crate::{DatabaseContext, RegisterUserRequest};

        pub fn not_blank(value: &str) -> Result<(), ValidationError> {
            if value.trim().is_empty() {
                return Err(ValidationError::new("not_blank")
                    .with_message(Cow::from("value cannot be blank")));
            }

            Ok(())
        }

        pub fn password_equals_confirm_password(
            request: &RegisterUserRequest,
        ) -> Result<(), ValidationError> {
            if request.password != request.confirm_password {
                return Err(ValidationError::new("password_equals_confirm_password")
                    .with_message(Cow::from("password and confirm password must be the same")));
            }

            Ok(())
        }

        pub fn can_register(
            request: &RegisterUserRequest,
            context: &DatabaseContext,
        ) -> Result<(), ValidationError> {
            if context.total >= context.max_data {
                return Err(
                    ValidationError::new("can_register").with_message(Cow::from(format!(
                        "cannot register user {}, database is full",
                        request.name
                    ))),
                );
            }

            Ok(())
        }
    }
}

pub struct DatabaseContext {
    total: i32,
    max_data: i32,
}

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Validate)]
struct LoginRequest {
    #[validate(length(
        min = 3,
        max = 20,
        message = "username must be between 3 and 20 characters"
    ))]
    username: String,

    #[validate(length(
        min = 3,
        max = 20,
        message = "password must be between 3 and 20 characters"
    ))]
    password: String,
}

#[derive(Debug, Validate)]
struct AddressRequest {
    #[validate(length(min = 1, max = 100))]
    street: String,

    #[validate(length(min = 1, max = 100))]
    city: String,

    #[validate(length(min = 1, max = 100))]
    country: String,
}

#[derive(Debug, Validate)]
#[validate(context=DatabaseContext,
    schema(
        function = "crate::validatorapp::validator::password_equals_confirm_password",
        skip_on_field_errors = false,
        code = "password",
        message = "password != confirm_password"
    ),
    schema(
        function = "crate::validatorapp::validator::can_register",
        skip_on_field_errors = false,
        code = "username",
        use_context
    )
)]
pub struct RegisterUserRequest {
    #[validate(length(
        min = 3,
        max = 20,
        message = "username must be between 3 and 20 characters"
    ))]
    username: String,

    #[validate(length(
        min = 3,
        max = 20,
        message = "password must be between 3 and 20 characters"
    ))]
    password: String,

    confirm_password: String,

    #[validate(length(
        min = 3,
        max = 100,
        message = "name must be between 3 and 100 characters"
    ))]
    name: String,

    #[validate(nested)]
    address: AddressRequest,
}

#[test]
fn test_validate_success() {
    let login = LoginRequest {
        username: "rizki".to_string(),
        password: "password".to_string(),
    };

    assert!(login.validate().is_ok());
}

#[test]
fn test_validate_failed() {
    let login = LoginRequest {
        username: "ri".to_string(),
        password: "p".to_string(),
    };

    assert!(login.validate().is_err());

    let errors = login.validate().err().unwrap();
    println!("{:?}", errors)
}

#[test]
fn test_validate_nested_struct() {
    let register = RegisterUserRequest {
        username: "rizki".to_string(),
        password: "password".to_string(),
        confirm_password: "password".to_string(),
        name: "Rizki Harahap".to_string(),
        address: AddressRequest {
            street: "Jl. Jalan".to_string(),
            city: "Jakarta".to_string(),
            country: "Indonesia".to_string(),
        },
    };

    let context = DatabaseContext {
        total: 100,
        max_data: 1000,
    };

    assert!(register.validate_with_args(&context).is_ok());
}

#[test]
fn test_validate_nested_invalid() {
    let register = RegisterUserRequest {
        username: "rizki".to_string(),
        confirm_password: "password".to_string(),
        password: "".to_string(),
        name: "Rizki Harahap".to_string(),
        address: AddressRequest {
            street: "".to_string(),
            city: "".to_string(),
            country: "".to_string(),
        },
    };

    let context = DatabaseContext {
        total: 100,
        max_data: 100,
    };

    let errors = register.validate_with_args(&context).err().unwrap();
    println!("{:?}", errors.errors())
}

#[derive(Debug, Validate)]
struct Product {
    #[validate(length(min = 3, max = 100,))]
    id: String,

    #[validate(length(min = 3, max = 100,))]
    name: String,

    #[validate(nested, length(min = 1,))]
    variants: Vec<ProductVariant>,
}

#[derive(Debug, Validate, Serialize)]
struct ProductVariant {
    #[validate(length(min = 3, max = 100,))]
    name: String,

    #[validate(range(min = 1, max = 1000000000))]
    price: i32,
}

#[test]
fn test_validate_vector() {
    let product = Product {
        id: "product-1".to_string(),
        name: "Product 1".to_string(),
        variants: vec![
            ProductVariant {
                name: "Variant 1".to_string(),
                price: 1000,
            },
            ProductVariant {
                name: "Variant 2".to_string(),
                price: 2000,
            },
        ],
    };

    assert!(product.validate().is_ok());
}

#[test]
fn test_validate_vector_invalid() {
    let product = Product {
        id: "product-1".to_string(),
        name: "Product 1".to_string(),
        variants: vec![
            ProductVariant {
                name: "".to_string(),
                price: -1000,
            },
            ProductVariant {
                name: "".to_string(),
                price: -2000,
            },
        ],
    };

    assert!(product.validate().is_err());

    let errors = product.validate().err().unwrap();
    println!("{:?}", errors)
}

#[derive(Debug, Validate)]
struct CreateCategoryRequest {
    #[validate(custom(function = "crate::validatorapp::validator::not_blank"))]
    id: String,

    #[validate(custom(function = "crate::validatorapp::validator::not_blank"))]
    name: String,
}

#[test]
fn test_custom_validation() {
    let category = CreateCategoryRequest {
        id: "".to_string(),
        name: "".to_string(),
    };

    let errors = category.validate().err().unwrap();
    println!("{:?}", errors.errors());
}
