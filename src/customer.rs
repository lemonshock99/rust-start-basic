pub struct Customer {
    name: String,
    age: u8,
}

impl Customer {

    // function in module
    pub fn new(name: String, age: u8) -> Self {
        Self {
            name: name,
            age: age,
        }
    }

    // Method
    pub fn hello(&self) {
        println!("Customer name is {} age {}",self.name, self.age);
    }
}



