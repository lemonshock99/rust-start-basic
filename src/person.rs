

pub struct Person {
    name: String,
    age: u8,
}


impl Person {

    // function in module
    pub fn new(name: String, age: u8) -> Self {
        Self {
            name: name,
            age: age,
        }
    }

    // Method
    pub fn hello(&self) {
        println!("My name is {}",self.name);
    }
}

