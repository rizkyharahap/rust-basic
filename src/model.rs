pub struct User {
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub age: u8,
}

impl User {
    pub fn say_hello(&self, name: &str) {
        println!("Hello {}, my name is {}", name, self.first_name);
    }
}
