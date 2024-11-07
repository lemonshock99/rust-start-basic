use std::char::from_u32;
use rand; // Need to add dependencies in cargo.toml 
use rand::{Rng, thread_rng};

pub fn gen_password() {

    let password_length:i32 = 15;
    let mut result = String::new();

    for _ in 0..password_length {
        let number = thread_rng().gen_range(48..122);
        let ch = from_u32(number).unwrap();

        result.push(ch);
    }

    println!("Auto generate password is: {}", result);
}

