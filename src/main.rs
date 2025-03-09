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
