use crate::third::say_hello as say_hello_third;

pub fn say_hello() {
    println!("Hello from first module");

    say_hello_third();
}

mod second {
    mod third {
        pub fn say_hello() {
            super::super::say_hello();
        }
    }
}
