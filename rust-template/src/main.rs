use std::collections::HashMap;

use handlebars::{Handlebars, HelperDef, handlebars_helper};
use serde::Serialize;
use serde_json::json;

struct DoubleNumber;

impl HelperDef for DoubleNumber {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &handlebars::Helper<'rc>,
        r: &'reg Handlebars<'reg>,
        ctx: &'rc handlebars::Context,
        rc: &mut handlebars::RenderContext<'reg, 'rc>,
        out: &mut dyn handlebars::Output,
    ) -> handlebars::HelperResult {
        let param = h.param(0).unwrap();
        let number = param.value().as_i64().unwrap();
        out.write(&format!("{}", number * 2));

        Ok(())
    }
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_handlebars() {
    let mut handlebars = Handlebars::new();

    handlebars
        .register_template_string("hello", "Hello, {{name}}")
        .unwrap();

    handlebars
        .register_template_string("bye", "Bye, {{name}}")
        .unwrap();

    let mut data = HashMap::new();
    data.insert("name", "Rizki");

    let rendered = handlebars.render("hello", &data).unwrap();
    assert_eq!(rendered, "Hello, Rizki");

    let rendered = handlebars.render("bye", &data).unwrap();
    assert_eq!(rendered, "Bye, Rizki");
}

#[test]
fn test_nested_variable() {
    let mut handlebars = Handlebars::new();

    handlebars
        .register_template_string("hello", "Hello, {{person.first_name}} {{person.last_name}}")
        .unwrap();

    let mut data = HashMap::new();
    let mut person = HashMap::new();
    person.insert("first_name", "Rizki");
    person.insert("last_name", "Harahap");
    data.insert("person", person);

    let rendered = handlebars.render("hello", &data).unwrap();
    assert_eq!(rendered, "Hello, Rizki Harahap");
}

#[test]
fn test_html_escape() {
    let mut handlebars = Handlebars::new();

    handlebars
        .register_template_string("hello", "Hello, {{{name}}}")
        .unwrap();

    let mut data = HashMap::new();
    data.insert("name", "<b>Rizki</b>");

    let rendered = handlebars.render("hello", &data).unwrap();
    assert_eq!(rendered, "Hello, <b>Rizki</b>");
}

#[test]
fn test_template_file() {
    let mut handlebars = Handlebars::new();

    handlebars
        .register_template_file("hello", "templates/hello.mustache")
        .unwrap();

    let mut data = HashMap::new();
    data.insert("name", "Rizki");

    let rendered = handlebars.render("hello", &data).unwrap();
    assert_eq!(rendered, "Hello, Rizki!");
}

#[test]
fn test_with() {
    let mut handlebars = Handlebars::new();

    handlebars
        .register_template_file("hello", "templates/with-hello.mustache")
        .unwrap();

    let mut data = HashMap::new();
    let mut person = HashMap::new();
    person.insert("first_name", "Rizki");
    person.insert("last_name", "Harahap");
    data.insert("person", person);

    let rendered = handlebars.render("hello", &data).unwrap();
    assert_eq!(rendered.contains("<h1>Hello, Rizki Harahap</h1>"), true);
}

#[derive(Serialize)]
struct Address {
    street: String,
    city: String,
}

#[derive(Serialize)]
struct Person {
    first_name: String,
    last_name: String,
    hobbies: Vec<String>,
    addresses: Vec<Address>,
}

#[derive(Serialize)]
struct Data {
    person: Person,
}

#[test]
fn test_serde_struct() {
    let mut handlebars = Handlebars::new();

    handlebars
        .register_template_file("hello", "templates/with-hello.mustache")
        .unwrap();

    let data = Data {
        person: Person {
            first_name: "Rizki".to_string(),
            last_name: "Harahap".to_string(),
            hobbies: vec![],
            addresses: vec![],
        },
    };

    let rendered = handlebars.render("hello", &data).unwrap();
    assert_eq!(rendered.contains("<h1>Hello, Rizki Harahap</h1>"), true);
}

#[test]
fn test_serde_json() {
    let mut handlebars = Handlebars::new();

    handlebars
        .register_template_file("hello", "templates/with-hello.mustache")
        .unwrap();

    let data = json!({
        "person": {
            "first_name":"Rizki",
            "last_name":"Harahap"
        }
    });

    let rendered = handlebars.render("hello", &data).unwrap();
    assert_eq!(rendered.contains("<h1>Hello, Rizki Harahap</h1>"), true);
}

#[test]
fn test_if() {
    let mut handlebars = Handlebars::new();

    handlebars
        .register_template_file("layout/header", "templates/layouts/header.mustache")
        .unwrap();
    handlebars
        .register_template_file("layout/footer", "templates/layouts/footer.mustache")
        .unwrap();
    handlebars
        .register_template_file("hello", "templates/blog.mustache")
        .unwrap();

    let data = json!({
        "title": "Learn Rust",
        "content": "learn rust",
        "footer": "Rizki Harahap"
    });

    let rendered = handlebars.render("hello", &data).unwrap();

    println!("{}", rendered);

    assert_eq!(rendered.contains("Learn Rust"), true);
    assert_eq!(rendered.contains("learn rust"), true);
    assert_eq!(rendered.contains("Anonymous"), true);
    assert_eq!(rendered.contains("Rizki Harahap"), true);
}

#[test]
fn test_if_else() {
    let mut handlebars = Handlebars::new();

    handlebars
        .register_template_file("hello", "templates/blog.mustache")
        .unwrap();

    let data = json!({
        "title":"Learn Rust",
        "content":"learn rust",
        "author": "Rizki"
    });

    let rendered = handlebars.render("hello", &data).unwrap();

    assert_eq!(rendered.contains("Learn Rust"), true);
    assert_eq!(rendered.contains("learn rust"), true);
    assert_eq!(rendered.contains("Anonymous"), false);
    assert_eq!(rendered.contains("Rizki"), true);
}

#[test]
fn test_unless() {
    let mut handlebars = Handlebars::new();

    handlebars
        .register_template_file("hello", "templates/footer.mustache")
        .unwrap();

    let data = json!({});

    let rendered = handlebars.render("hello", &data).unwrap();

    assert_eq!(
        rendered.contains("This content does not contains footer"),
        true
    );

    let data = json!({
        "footer": "Footer"
    });

    let rendered = handlebars.render("hello", &data).unwrap();

    assert_eq!(
        rendered.contains("This content does not contains footer"),
        false
    );
}

#[test]
fn test_each() {
    let mut handlebars = Handlebars::new();

    handlebars
        .register_template_file("hello", "templates/person.mustache")
        .unwrap();

    let data = Person {
        first_name: "Rizki".to_string(),
        last_name: "Harahap".to_string(),
        hobbies: vec!["Coding".to_string(), "Gaming".to_string()],
        addresses: vec![
            Address {
                street: "Jl. Jalan".to_string(),
                city: "Jakarta".to_string(),
            },
            Address {
                street: "Jl. Halan".to_string(),
                city: "Bandung".to_string(),
            },
        ],
    };

    let rendered = handlebars.render("hello", &data).unwrap();

    assert_eq!(rendered.contains("Rizki"), true);
    assert_eq!(rendered.contains("Harahap"), true);
    assert_eq!(rendered.contains("0 - Coding"), true);
    assert_eq!(rendered.contains("1 - Gaming"), true);
    assert_eq!(rendered.contains("street - Jl. Jalan"), true);
    assert_eq!(rendered.contains("city - Jakarta"), true);
    assert_eq!(rendered.contains("street - Jl. Halan"), true);
    assert_eq!(rendered.contains("city - Bandung"), true);
}

#[test]
fn test_helper() {
    let mut handlebars = Handlebars::new();

    handlebars.register_helper("double", Box::new(DoubleNumber));
    handlebars
        .register_template_string("helper", "Result : {{double value}}")
        .unwrap();

    let data = json!({
        "value": 10
    });

    let rendered = handlebars.render("helper", &data).unwrap();

    assert_eq!(rendered.contains("Result : 20"), true);
}

handlebars_helper!(uppercase: | value:String | value.to_uppercase());

#[test]
fn test_helper_macro() {
    let mut handlebars = Handlebars::new();

    handlebars.register_helper("uppercase", Box::new(uppercase));
    handlebars
        .register_template_string("helper", "Hello {{uppercase name}}")
        .unwrap();

    let data = json!({
        "name": "Rizki"
    });

    let rendered = handlebars.render("helper", &data).unwrap();

    assert_eq!(rendered.contains("Hello RIZKI"), true);
}
