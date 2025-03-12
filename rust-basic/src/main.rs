mod first;
mod model;
mod second;
mod third;

use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    fmt::{Debug, Formatter},
    ops::{Add, Deref},
    rc::Rc,
    result,
};

use first::say_hello;
use second::say_hello as say_hello_second;

#[test]
fn test_use() {
    say_hello();
    say_hello_second();
}

#[test]
fn test_module() {
    let user = model::User {
        first_name: String::from("Rizki"),
        middle_name: String::from("Mahfuddin"),
        last_name: String::from("Harahap"),
        age: 23,
    };

    user.say_hello("Budi");
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_hello() {
    println!("Hello Test");
}

#[test]
fn test_variable() {
    let name = "Rizki Harahap";
    println!("Hello {}", name);
}

#[test]
fn test_mutable() {
    let mut name = "Rizki Harahap";
    println!("Hello {}", name);

    name = "Haphap";
    println!("Hello {}", name);
}

#[test]
fn test_static_typing() {
    let name = "Rizki Harahap";
    println!("Hello {}", name);

    // name = "Haphap";
    println!("Hello {}", name);
}

#[test]
fn test_shadowing() {
    let name = "Rizki Harahap";
    println!("Hello {}", name);

    let name = "Haphap";
    println!("Hello {}", name);
}

#[test]
fn test_explicit() {
    let age: i32 = 1;
    println!("{}", age);
}

#[test]
fn test_number_conversion() {
    let a: i8 = 10;
    println!("{}", a);

    let b: i16 = a as i16;
    println!("{}", b);

    let c: i32 = b as i32;
    println!("{}", c);

    let d: i64 = 1000000000;
    println!("{}", d);

    // Integer overflow causes value is bigger than i8
    let e: i8 = d as i8;
    println!("{}", e);
}

#[test]
fn test_augmented_assignment() {
    let mut a = 10;
    println!("{}", a);

    a += 10;
    println!("{}", a);

    a -= 10;
    println!("{}", a);
}

#[test]
fn test_char_type() {
    let char1 = 'a';
    let char2 = 'b';
    println!("{} {}", char1, char2);
}

#[test]
fn test_tuple() {
    let data: (i32, f64, bool) = (10, 10.5, true);
    println!("{:?}", data);
    println!("{}", data.0);
    println!("{}", data.1);
    println!("{}", data.2);

    // destructuring tuple
    let (a, b, _) = data;
    println!("{} {}", a, b);
}

#[test]
fn test_mutable_tuple() {
    let mut data: (i32, f64, bool) = (10, 10.5, true);
    println!("{:?}", data);

    data.0 = 20;
    data.1 = 20.5;
    data.2 = false;

    println!("{}", data.0);
    println!("{}", data.1);
    println!("{}", data.2);
}

fn unit() {
    println!("Hello");
}

#[test]
fn test_unit() {
    let result = unit();
    println!("{:?}", result);

    let test = ();
    println!("{:?}", test);
}

#[test]
fn test_array() {
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", array);

    let a = array[0];
    let b = array[1];
    println!("{} {}", a, b);

    // destructuring array
    let [a, b, ..] = array;
    println!("{} {}", a, b);

    let [a, b, rest @ ..] = array;
    println!("{} {}", a, b);
    println!("{:?}", rest);
}

#[test]
fn test_mutable_array() {
    let mut array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", array);

    array[0] = 10;
    array[1] = 20;
    println!("{:?}", array);

    // check length of array
    let length: usize = array.len();
    println!("{}", length);
}

#[test]
fn test_two_dimensional_array() {
    let matrix: [[i32; 2]; 2] = [[1, 2], [3, 4]];
    println!("{:?}", matrix);
    println!("{:?}", matrix[0]);
    println!("{:?}", matrix[1]);
    println!("{:?}", matrix[0][0]);
    println!("{:?}", matrix[0][1]);
}

const MAXIMUM: i32 = 0;

#[test]
fn test_constant() {
    const MINIMUM: i32 = 0;
    println!("{} {}", MINIMUM, MAXIMUM);
}

#[test]
fn test_variable_scope() {
    let a = 1; // variable_scope

    {
        // inner scope
        println!("inner a: {}", a);

        let b = 2;
        println!("inner b: {}", b);
    }

    println!("inner a: {}", a);
}

#[test]
fn test_str() {
    let name: &str = " Rizki Harahap  ";
    let trim: &str = name.trim();

    println!("{}", name);
    println!("{}", trim);

    let mut username: &str = "Rizki";
    username = "Budi";
    println!("{}", username);
}

#[test]
fn test_string_type() {
    let mut name: String = String::from("Rizki");
    println!("{}", name);

    name.push_str(" Harahap");
    println!("{}", name);

    name.replace("Rizki", "Budi");
    println!("{}", name);

    let budi = name.replace("Rizki", "Budi");
    println!("{}", budi);
}

#[test]
fn test_ownership_rules() {
    // a can't accessed from this line, causes not declarated
    let a = 10; // a can accessed start from this line

    {
        // b can't accessed from this line, causes not declarated
        let b = 20;
        println!("{}", b);
    } // scope b ended, b will removed, b can't accessible anymore

    println!("{}", a);
} // scope a ended, a will removed, a can't accessible anymore

#[test]
fn test_data_copy() {
    let a = 10;
    let b = a; // copy data from a to b

    println!("{} {}", a, b);
}

#[test]
fn test_ownership_movement() {
    let name1 = String::from("Rizki");
    println!("{}", name1);

    // ownership from name1 moved to name2
    let name2 = name1;
    // name 1 can't accessed from this line anymore

    // println!("{}", name1); // can't accessed anymore,causes ownership belongs to name2
    println!("{}", name2);
}

#[test]
fn test_clone() {
    let name1 = String::from("Rizki");
    let name2 = name1.clone();

    println!("{} {}", name1, name2);
}

#[test]
fn test_if_expression() {
    let value = 9;

    let result = if value >= 8 {
        "Good"
    } else if value >= 6 {
        "Not Good"
    } else {
        "Very Bad"
    };

    print!("{}", result);
}

#[test]
fn test_loop_expression() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        println!("{}", counter);

        if counter > 10 {
            break counter * 2;
        } else if counter % 2 == 0 {
            continue;
        }
    };

    println!("{}", result)
}

#[test]
fn test_loop_label() {
    let mut number = 1;

    'outer: loop {
        let mut i = 1;

        loop {
            if number > 10 {
                break 'outer;
            }

            println!("{} x {} = {}", number, i, number * i);

            i += 1;

            if i > 10 {
                break;
            }
        }

        number += 1;
    }
}

#[test]
fn test_while_loop() {
    let mut counter = 0;

    while counter <= 10 {
        if counter % 2 == 0 {
            println!("Counter : {}", counter);
        }

        counter += 1;
    }
}

#[test]
fn test_array_iteration() {
    let array = ["A", "B", "C", "D", "E"];
    let mut index = 0;

    while index < array.len() {
        println!("value : {}", array[index]);

        index += 1;
    }

    for value in array {
        println!("value : {}", value);
    }
}

#[test]
fn test_range() {
    let range = 0..5;
    println!("Start : {}", range.start);
    println!("End : {}", range.end);

    for i in range {
        println!("value : {}", i);
    }
}

#[test]
fn test_range_inclusive() {
    let range = 0..=5;
    println!("Start : {}", range.start());
    println!("End : {}", range.end());

    for i in range {
        println!("value : {}", i);
    }
}

fn factorial_loop(n: i32) -> i32 {
    if n < 1 {
        return 0;
    }

    let mut result = 1;
    for i in 1..=n {
        result *= i;
    }

    return result;
}

#[test]
fn test_factorial_loop() {
    let result = factorial_loop(5);
    println!("{}", result);

    let result = factorial_loop(10);
    println!("{}", result);
}

fn print_text(value: String, times: u32) {
    if times == 0 {
        return;
    } else {
        println!("{}", value);
    }

    print_text(value, times - 1);
}

#[test]
fn test_print_text() {
    print_text(String::from("Rizki"), 5);
}

fn factorial_recursive(n: u32) -> u32 {
    if n <= 1 {
        return 1;
    }

    return n * factorial_recursive(n - 1);
}

#[test]
fn test_factorial_recursive() {
    let result = factorial_loop(5);
    println!("{}", result);
}

fn print_number(number: i32) {
    println!("number : {}", number);
}

fn hi(name: String) {
    println!("Hi, {}", name);
}

#[test]
fn test_ownership_parameter() {
    let number = 10;
    print_number(number);
    println!("{}", number);

    let name = String::from("Rizki");
    hi(name);
    // println!("{}", name); // cant accessed name causes name is moved the ownership to hi()
}

fn full_name(first_name: String, last_name: String) -> (String, String, String) {
    let full_name = format!("{} {}", first_name, last_name);

    return (first_name, last_name, full_name);
}

#[test]
fn test_return_value_ownership() {
    let first_name = String::from("Rizki");
    let last_name = String::from("Harahap");

    let (first_name, last_name, full_name) = full_name(first_name, last_name);

    println!("{}", full_name);
    println!("{}", first_name);
    println!("{}", last_name);
}

fn full_name_reference(first_name: &String, last_name: &String) -> String {
    let full_name = format!("{} {}", first_name, last_name);

    return full_name;
}

#[test]
fn test_reference() {
    let first_name = String::from("Rizki");
    let last_name = String::from("Harahap");

    let full_name = full_name_reference(&first_name, &last_name);

    println!("{}", full_name);
    println!("{}", first_name);
    println!("{}", last_name);
}

fn chance_value(value: &mut String) {
    value.push_str(" Test");
}

#[test]
fn test_value_borrowing() {
    let mut value = String::from("Rizki");

    chance_value(&mut value);
    println!("{}", value);
}

// fn get_full_name(first_name: &String, last_name: &String) -> &String {
//     let full_name = format!("{} {}", first_name, last_name);

//     return &full_name;
// }

// #[test]
// fn test_dangling_pointer() {
//     let first_name = String::from("Rizki");
//     let last_name = String::from("Harahap");

//     let full_name = get_full_name(&first_name, &last_name);

//     println!("{}", full_name);
//     println!("{}", first_name);
//     println!("{}", last_name);
// }

#[test]
fn test_slice_reference() {
    let array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let slice1 = &array[..];
    println!("{:?}", slice1);

    let slice2 = &array[0..5];
    println!("{:?}", slice2);

    let slice3 = &array[5..];
    println!("{:?}", slice3);
}

struct Nothing;

#[test]
fn test_struct_without_field() {
    let _nothing1 = Nothing;
    let _nothing2 = Nothing {};
}

struct Person {
    first_name: String,
    middle_name: String,
    last_name: String,
    age: u8,
}

impl Person {
    fn say_hello(&self, name: &str) {
        println!("Hello {}, my name is {}", name, self.first_name);
    }
}

fn print_person(person: &Person) {
    println!(
        "{} {} {}",
        person.first_name, person.middle_name, person.last_name
    );

    println!("{}", person.age);
}

#[test]
fn test_struct_person() {
    let person = Person {
        first_name: String::from("Rizki"),
        middle_name: String::from("Mahfuddin"),
        last_name: String::from("Harahap"),
        age: 25,
    };

    print_person(&person);

    let person2 = Person { ..person };
    print_person(&person2);

    let person3 = Person {
        first_name: person2.first_name.clone(),
        middle_name: person2.middle_name.clone(),
        last_name: person2.last_name.clone(),
        ..person2
    };
    print_person(&person2);
    print_person(&person3);
}

#[test]
fn test_method() {
    let person = Person {
        first_name: String::from("Rizki"),
        middle_name: String::from("Mahfuddin"),
        last_name: String::from("Harahap"),
        age: 25,
    };

    person.say_hello("Madhon");
    print_person(&person);
}

struct GeoPoint(f64, f64);

impl GeoPoint {
    fn new(long: f64, lat: f64) -> GeoPoint {
        return GeoPoint(long, lat);
    }
}

#[test]
fn test_tuple_struct() {
    let geo_point = GeoPoint(-6.234324, 100.34343);
    println!("{}", geo_point.0);
    println!("{}", geo_point.1);
}

#[test]
fn test_associated_function() {
    let geo_point = GeoPoint::new(10.0, 10.0);
    println!("{}", geo_point.0);
    println!("{}", geo_point.1);
}

enum Level {
    Regular,
    Premium,
    Platinum,
}

#[test]
fn test_enum() {
    let level = Level::Premium;

    match level {
        Level::Regular => {
            println!("Regular")
        }

        Level::Premium => {
            println!("Premium")
        }

        Level::Platinum => {
            println!("Platinum")
        }
    }
}

enum Payment {
    // card number
    CreditCart(String),
    // bank name, account number
    BankTransfer(String, String),
    // ewallet name, ewallet number
    EWallet(String, i32),
}

impl Payment {
    fn pay(&self, amount: u32) {
        match self {
            Payment::CreditCart(number) => {
                println!("Paying with credit card {} amount {}", number, amount);
            }
            Payment::BankTransfer(bank, number) => {
                println!(
                    "Paying with bank transfer {} {} amount {}",
                    bank, number, amount
                );
            }
            Payment::EWallet(wallet, number) => {
                println!(
                    "Paying with e-wallet {} {} amount {}",
                    wallet, number, amount
                );
            }
        }

        println!("Paying amount {}", amount,)
    }
}

#[test]
fn test_enum_data() {
    let _payment1 = Payment::CreditCart(String::from("1234567890"));
    _payment1.pay(10000000);

    let _payment2 = Payment::BankTransfer(String::from("BCA"), String::from("0987654"));
    _payment2.pay(20000000);

    let _payment3 = Payment::EWallet(String::from("Gopay"), 12343242);
    _payment3.pay(30000000);
}

#[test]
fn test_match_value() {
    let name = "Joko";

    match name {
        "Rizki" | "Riski" | "Rizky" => {
            println!("Hello Rizki");
        }
        "Harahap" => {
            println!("Hello Rizki");
        }
        _ => {
            println!("Who are you?");
        }
    }
}

#[test]
fn test_range_patterns() {
    let value = 100;

    let grade = match value {
        75..=100 => "Great",
        50..=74 => "Good",
        25..=49 => "Not Bad",
        0..=24 => "Bad",
        _ => "Invalid Value",
    };

    println!("{}", grade);
}

#[test]
fn test_struct_patterns() {
    let point = GeoPoint::new(0.0, 1.0);

    match point {
        GeoPoint(long, 0.0) => {
            println!("long : {}", long);
        }

        GeoPoint(0.0, lat) => {
            println!("at : {}", lat);
        }

        GeoPoint(long, lat) => {
            println!("long : {}, lat : {}", long, lat);
        }
    }

    let person = Person {
        first_name: String::from("Rizki"),
        middle_name: String::from("Mahfuddin"),
        last_name: String::from("Harahap"),
        age: 25,
    };

    match person {
        Person {
            first_name,
            last_name,
            ..
        } => {
            println!("First Name: {}, Last Name: {}", first_name, last_name)
        }
    }
}

type Age = u8;
type IdentityNumber = String;

struct Customer {
    id: IdentityNumber,
    name: String,
    age: Age,
}

type Pelanggan = Customer;

#[test]
fn test_type_alias() {
    let customer = Customer {
        id: String::from("12343242"),
        name: String::from("Rizki"),
        age: 20,
    };

    println!("{} {} {}", customer.id, customer.name, customer.age);
}

trait CanSayHello {
    fn hello(&self) -> String {
        return String::from("Hello matafaka");
    }

    fn say_hello(&self) -> String;
    fn say_hello_to(&self, name: &str) -> String;
}

impl CanSayHello for Person {
    fn say_hello(&self) -> String {
        return format!("Hello, my name is {}", self.first_name);
    }

    fn say_hello_to(&self, name: &str) -> String {
        return format!("Hello {}, my name is {}", name, self.first_name);
    }
}

trait CanSayGoodbye {
    fn say_goodbye(&self) -> String;
    fn say_goodbye_to(&self, name: &str) -> String;
}

impl CanSayGoodbye for Person {
    fn say_goodbye(&self) -> String {
        return format!("Goodbye, my name is {}", self.first_name);
    }

    fn say_goodbye_to(&self, name: &str) -> String {
        return format!("Goodbye {}, my name is {}", name, self.first_name);
    }
}

fn say_hello_trait(value: &impl CanSayHello) {
    println!("{}", value.say_hello());
}

fn hello_and_goodbye(value: &(impl CanSayHello + CanSayGoodbye)) {
    println!("{}, {}", value.say_hello(), value.say_goodbye())
}

#[test]
fn test_trait() {
    let person = Person {
        first_name: String::from("Rizki"),
        middle_name: String::from("Mahfuddin"),
        last_name: String::from("Harahap"),
        age: 25,
    };

    println!("{}", person.say_hello_to("Budi"));
    println!("{}", person.hello());

    say_hello_trait(&person);

    println!("{}", person.say_goodbye());
    println!("{}", person.say_goodbye_to("Budi"));

    hello_and_goodbye(&person);

    println!("{}", CanSayHello::say_hello(&person));
    Person::say_hello(&person, "Budi");
}

struct SimplePerson {
    name: String,
}

impl CanSayGoodbye for SimplePerson {
    fn say_goodbye(&self) -> String {
        return format!("Goodbye, my name is {}", self.name);
    }

    fn say_goodbye_to(&self, name: &str) -> String {
        return format!("Goodbye {}, my name is {}", name, self.name);
    }
}

fn create_person(name: String) -> impl CanSayGoodbye {
    return SimplePerson { name };
}

#[test]
fn test_return_trait() {
    let person = create_person(String::from("Rizki"));

    println!("{}", person.say_goodbye());
    println!("{}", person.say_goodbye_to("Budi"));
}

trait CanSay: CanSayHello + CanSayGoodbye {
    fn say(&self) {}
}

struct SimpleMan {
    name: String,
}

impl CanSayHello for SimpleMan {
    fn say_hello(&self) -> String {
        return format!("Hello, my name is {}", self.name);
    }

    fn say_hello_to(&self, name: &str) -> String {
        return format!("Hello {}, my name is {}", name, self.name);
    }
}

impl CanSayGoodbye for SimpleMan {
    fn say_goodbye(&self) -> String {
        return format!("Goodbye, my name is {}", self.name);
    }

    fn say_goodbye_to(&self, name: &str) -> String {
        return format!("Goodbye {}, my name is {}", name, self.name);
    }
}

impl CanSay for SimpleMan {
    fn say(&self) {
        println!("{}", self.say_hello());
        println!("{}", self.say_goodbye());
    }
}

#[test]
fn test_super_trait() {
    let person = SimpleMan {
        name: String::from("Rizki"),
    };

    person.say();
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn get_x(&self) -> &T {
        return &self.x;
    }

    fn get_y(&self) -> &T {
        return &self.y;
    }
}

#[test]
fn test_generic_struct() {
    let _integer = Point::<i32> { x: 60, y: 110 };
    let _float = Point::<f64> {
        x: 60.21342,
        y: 110.23423,
    };
}

enum Value<T> {
    NONE,
    VALUE(T),
}

#[test]
fn test_generic_enum() {
    let value = Value::<i32>::VALUE(10);

    match value {
        Value::NONE => {
            println!("none");
        }
        Value::VALUE(value) => {
            println!("value {}", value);
        }
    }
}

struct Hi<T: CanSayGoodbye> {
    value: T,
}

#[test]
fn test_generic_struct_with_trait() {
    let hi = Hi::<SimplePerson> {
        value: SimplePerson {
            name: String::from("Rizki"),
        },
    };

    println!("{}", hi.value.name);
}

fn min<T: PartialOrd>(value1: T, value2: T) -> T {
    if value1 < value2 {
        return value1;
    } else {
        return value2;
    }
}

#[test]
fn test_generic_function() {
    let value = min::<i32>(20, 10);

    println!("{}", value);
}

struct Test {
    a: i32,
}

#[test]
fn test_generic_method() {
    let point = Point { x: 7, y: 20 };
    println!("{}", point.get_x());
    println!("{}", point.get_y());
    println!("{}", point.get_value());

    let point_str = Point::<Test> {
        x: Test { a: 12 },
        y: Test { a: 12 },
    };
    println!("{}", point_str.get_x().a);
}

trait GetValue<T>
where
    T: PartialOrd,
{
    fn get_value(&self) -> &T;
}

impl<T> GetValue<T> for Point<T>
where
    T: PartialOrd,
{
    fn get_value(&self) -> &T {
        return &self.x;
    }
}

struct Apple {
    quantity: i32,
}

impl Add for Apple {
    type Output = Apple;

    fn add(self, rhs: Self) -> Self::Output {
        return Apple {
            quantity: self.quantity + rhs.quantity,
        };
    }
}

#[test]
fn test_operator_add() {
    let apple1 = Apple { quantity: 10 };
    let apple2 = Apple { quantity: 20 };

    let result = apple1 + apple2;

    println!("{}", result.quantity);
}

fn double(value: Option<i32>) -> Option<i32> {
    match value {
        None => None,
        Some(i) => Some(1 * 2),
    }
}

#[test]
fn test_option() {
    let result = double(Option::Some(10));
    println!("{:?}", result);

    let result = double(Option::None);
    println!("{:?}", result);
}

struct Category {
    id: String,
    name: String,
}

impl Debug for Category {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Category")
            .field("id", &self.id)
            .field("name", &self.name)
            .finish()
    }
}

#[test]
fn test_formatting() {
    let category = Category {
        id: String::from("GADGET"),
        name: String::from("Gadget"),
    };

    println!("{:?}", category)
}

#[test]
fn test_closure() {
    let sum = |val1: i32, val2: i32| -> i32 {
        return val1 + val2;
    };

    let result = sum(1, 2);
    println!("{}", result);
}

fn print_with_filter(value: String, filter: fn(String) -> String) {
    let result = filter(value);
    println!("Result: {}", result);
}

#[test]
fn feature() {
    print_with_filter(String::from("Rizki"), |value: String| -> String {
        return value.to_uppercase();
    });
}

fn to_uppercase(value: String) -> String {
    return value.to_uppercase();
}

fn print_string(value1: String, value2: String) {
    println!("value1: {}, value2 {}", value1, value2);
}

#[test]
fn test_function_as_closure() {
    print_with_filter(String::from("Rizki"), to_uppercase);

    print_string(String::from("Rizki"), to_uppercase(String::from("Rizki")));
}

#[test]
fn test_vector() {
    let mut names = Vec::<String>::new();
    names.push(String::from("Rizki"));
    names.push(String::from("Mahfuddin"));
    names.push(String::from("Harahap"));

    for name in &names {
        println!("{}", name)
    }
}

#[test]
fn test_hash_map() {
    let mut map: HashMap<String, String> = HashMap::new();

    map.insert(String::from("name"), String::from("Rizki"));
    map.insert(String::from("age"), String::from("25"));

    let name = map.get("name");
    let age = map.get("age");

    println!("{} {}", name.unwrap(), age.unwrap());
}

#[test]
fn test_hash_set() {
    let mut set: HashSet<String> = HashSet::new();

    set.insert(String::from("Rizki"));
    set.insert(String::from("Rizki"));
    set.insert(String::from("Mahfuddin"));
    set.insert(String::from("Mahfuddin"));
    set.insert(String::from("Harahap"));
    set.insert(String::from("Harahap"));

    for value in &set {
        println!("{}", value)
    }
}

#[test]
fn test_iterator() {
    let array = [1, 2, 3, 4, 5];
    let mut iterator = array.iter();

    while let Some(value) = iterator.next() {
        println!("{}", value);
    }

    for value in iterator {
        println!("{}", value);
    }
}

#[test]
fn test_iterator_method() {
    let vector = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("Vector: {:?}", vector);

    let sum: i32 = vector.iter().sum();
    println!("Sum: {:?}", sum);

    let count = vector.iter().count();
    println!("Count: {:?}", count);

    let double: Vec<i32> = vector.iter().map(|x| x * 2).collect();
    println!("Double: {:?}", double);

    let odd: Vec<&i32> = vector.iter().filter(|x| *x % 2 != 0).collect();
    println!("Odd: {:?}", odd);
}

fn connect_database(host: Option<String>) {
    match host {
        None => {
            panic!("No database host provided");
        }
        Some(host) => {
            println!("Connecting to database at {}", host);
        }
    }
}

#[test]
fn test_unrecoverable_error_handling() {
    connect_database(Some(String::from("localhost")));
    // connect_database(None); // will panic
}

fn connect_cache(host: Option<String>) -> Result<String, String> {
    match host {
        None => Err("No cache host provided".to_string()),
        Some(host) => Ok(host),
    }
}

#[test]
fn test_recoverable_error_handling() {
    let cache = connect_cache(Some("localhost".to_string()));

    // let cache = connect_cache(None);

    match cache {
        Ok(host) => {
            println!("Success connect to host: {}", host);
        }

        Err(error) => {
            println!("Error with message: {}", error);
        }
    }
}

fn connect_email(host: Option<String>) -> Result<String, String> {
    match host {
        None => Err("No email host provided".to_string()),
        Some(host) => Ok(host),
    }
}

fn connect_application(host: Option<String>) -> Result<String, String> {
    // let cache = connect_cache(host.clone());

    // match cache {
    //     Ok(host) => {
    //         println!("Success connect to host: {}", host);
    //     }
    //     Err(error) => {
    //         return Err(error);
    //     }
    // };

    // let email = connect_email(host.clone());

    // match email {
    //     Ok(host) => {
    //         println!("Success connect to host: {}", host);
    //     }
    //     Err(error) => {
    //         return Err(error);
    //     }
    // };

    connect_cache(host.clone())?;
    connect_email(host.clone())?;

    return Ok("Connected to application".to_string());
}

#[test]
fn test_question_mark_operator() {
    let result = connect_application(Some("localhost".to_string()));

    // let result = connect_application(None);

    match result {
        Err(err) => {
            println!("Error with message: {}", err)
        }
        Ok(host) => {
            println!("Success connect with application: {}", host)
        }
    };
}

#[test]
fn test_dangling_reference() {
    let r: &i32;

    {
        let x = 3;
        // r = &x; // will be error cause borrowed value does not live long enough
    }

    r = &4;

    println!("r = {}", r);
}

fn longest<'a>(value1: &'a str, value2: &'a str) -> &'a str {
    if value1.len() > value2.len() {
        return value1;
    }

    return value2;
}

#[test]
fn test_lifetime_annotation() {
    let value1 = "Rizki";
    let value2 = "Harahap";
    let result = longest(value1, value2);

    println!("result: {}", result);
}

struct Student<'a, 'b> {
    name: &'a str,
    last_name: &'b str,
}

impl<'a, 'b> Student<'a, 'b> {
    fn longest_name(&self, student: &Student<'a, 'b>) -> &'a str {
        if self.name.len() > student.name.len() {
            return self.name;
        }

        return student.name;
    }
}

fn longest_student_name<'a, 'b>(student1: &Student<'a, 'b>, student2: &Student<'a, 'b>) -> &'a str {
    if student1.name.len() > student2.name.len() {
        return student1.name;
    }

    return student2.name;
}

#[test]
fn test_lifetime_annotation_on_struct() {
    let student1 = Student {
        name: "Rizki",
        last_name: "Rizki",
    };
    let student2 = Student {
        name: "Harahap",
        last_name: "Harahap",
    };
    let result = longest_student_name(&student1, &student2);
    println!("result: {}", result);

    let result = student1.longest_name(&student2);
    println!("result: {}", result);
}

struct Teacher<'a, ID>
where
    ID: Ord,
{
    id: ID,
    name: &'a str,
}

#[test]
fn test_lifetime_annotation_on_generic() {
    let teacher = Teacher::<i32> {
        id: 10,
        name: "Rizki",
    };

    println!("{}", teacher.id);
    println!("{}", teacher.name);
}

#[derive(Debug, PartialEq, PartialOrd)]
struct Company {
    name: String,
    location: String,
    website: String,
}

#[test]
fn test_attribute_debug() {
    let company1 = Company {
        name: "Rust".to_string(),
        location: "Indonesia".to_string(),
        website: "Website".to_string(),
    };

    let company2 = Company {
        name: "Rust".to_string(),
        location: "Indonesia".to_string(),
        website: "Website".to_string(),
    };

    let company3 = Company {
        name: "Rust".to_string(),
        location: "Indonesia".to_string(),
        website: "Website".to_string(),
    };

    println!("{:?}", company1);
    println!("{}", company1 == company2);
    println!("{}", company1 > company2);
    println!("{}", company2 > company3);
}

fn display_number(value: i32) {
    println!("value: {}", value);
}

fn display_number_reference(value: &i32) {
    println!("value: {}", value);
}

#[test]
fn test_box() {
    let value = Box::new(10);
    println!("value: {}", value);
    display_number(*value);
    display_number_reference(&value);
}

#[derive(Debug)]
enum ProductCategory {
    Of(String, Box<ProductCategory>),
    End,
}

#[test]
fn test_box_enum() {
    let category = ProductCategory::Of(
        "Laptop".to_string(),
        Box::new(ProductCategory::Of(
            "Dell".to_string(),
            Box::new(ProductCategory::End),
        )),
    );

    println!("{:?}", category);
}

#[test]
fn test_derefence() {
    let value1 = Box::new(10);
    let value2 = Box::new(20);

    let result = *value1 * *value2;

    println!("{}", result);
}

struct MyValue<T> {
    value: T,
}

impl<T> Deref for MyValue<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        return &self.value;
    }
}

#[test]
fn test_derefence_struct() {
    let value = MyValue { value: 10 };

    let real_value = *value;

    println!("{}", real_value);
}

fn say_hello_reference(name: &String) {
    println!("Hello, {}", name);
}

#[test]
fn test_deref_reference() {
    let name = MyValue {
        value: "Rizki".to_string(),
    };
    say_hello_reference(&name);
}

struct Book {
    title: String,
}

impl Drop for Book {
    fn drop(&mut self) {
        println!("Dropping book {}", &self.title);
    }
}

#[test]
fn test_drop() {
    let book = Book {
        title: "Rust Programming".to_string(),
    };
    println!("{}", book.title);
}

enum Brand {
    Of(String, Rc<Brand>),
    End,
}

#[test]
fn test_multiple_ownership() {
    let apple = Rc::new(Brand::Of("Apple".to_string(), Rc::new(Brand::End)));
    println!("Apple reference count: {}", Rc::strong_count(&apple));

    let laptop = Brand::Of("Laptop".to_string(), Rc::clone(&apple));
    println!("Apple reference count: {}", Rc::strong_count(&apple));

    {
        let smarthphone = Brand::Of("Smarthphone".to_string(), Rc::clone(&apple));
        println!("Apple reference count: {}", Rc::strong_count(&apple));
    }

    println!("Apple reference count: {}", Rc::strong_count(&apple));
}

#[derive(Debug)]
struct Seller {
    name: RefCell<String>,
    active: RefCell<bool>,
}

#[test]
fn test_ref_cell() {
    let seller = Seller {
        name: RefCell::new("Rizki".to_string()),
        active: RefCell::new(true),
    };

    {
        let mut result = seller.name.borrow_mut();

        println!("{}", result);

        *result = "Budi".to_string();

        println!("{}", result);
    }

    println!("{:?}", seller);
}

// static APPLICATION: &str = "My Application";

// #[test]
// fn test_static() {
//     println!("{}", APPLICATION);
// }

// static mut COUNTER: u32 = 0;

// unsafe fn increment() {
//     COUNTER += 1;
// }

// #[test]
// fn test_static_mut() {
//     unsafe {
//         increment();
//         COUNTER += 1;

//         println!("Counter : {}", COUNTER);
//     }
// }

macro_rules! hi {
    () => {
        println!("Hi macro!");
    };

    ($name: expr) => {
        println!("Hi {}!", $name);
    };
}

#[test]
fn test_macro() {
    hi!();
    hi!("Eko");
    hi! {
        "Eko"
    };

    let name = "Rizki";
    hi!(name);
}

macro_rules! iterate {
    ($array:expr) => {
        for i in $array {
            println!("{}", i);
        }
    };

    ($( $item:expr), *) => {
        $(
            println!("{}", $item);
        )*
    };
}

#[test]
fn test_macro_iterate() {
    iterate!([1, 2, 3, 4, 5, 6, 7, 8, 9]);
    iterate!(1, 2, 3, 4, 5, 6, 7, 8, 9);
}
