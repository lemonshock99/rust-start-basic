#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(while_true)]

use std::collections::HashMap;
// use hello_world::Person;
////////////
// use hello_world::person::Person;
// use hello_world::customer::Customer;
use hello_world::{
    customer::Customer,
    person::Person,
    speaking::Speaking
};

fn main() {
    //variavles 
    let mut x: i32;
    x = 1;

    let y =x;

    x = 100;
    println!("simple variable 1:{}",x);
    println!("simple variable 2:{}",y);

    //tuple

    let x: (i32, f64, i32) = (1, 3.14, 1000);

    println!("tuple variable: {}",x.0);

    //array
    let x: [i32; 5];
    let x: [i32; 5] = [1,2,3,4,5];
    let x: [i32; 5] = [0;5];

    println!("array variable: {:?}", x);

    //function
    println!("print function: {}",get_number());

    //if
    let score: i32 = 50;
    let grade: &str;
    if score >= 80{
        grade = "A";
    } else if score >= 70 {
        grade = "B";
    } else if score >= 60 {
        grade = "C";
    } else if score >= 50 {
        grade = "D";
    } else {
        grade = "F";
    };

    println!("Standard if else 1: {}",grade);

    //if 2

    let grade2= if score >= 80{
        "A"
    } else if score >= 70 {
        "B"
    } else if score >= 60 {
        "C"
    } else if score >= 50 {
        "D"
    } else {
        "F"
    };

    println!("Standard if else 2: {}", grade2);

    //if 3
    let result = if score >= 50 {"Pass"} else {"Fail"};
    println!("Standard if else 3: {}", result);

    //Loop
    let mut x = 0;
    while true {
        println!("while loop (not recommand): {}",x);
        x += 1;
        if x >= 3 {
            break;
        }
    }

    // prefer use loop because can use label
    'label_loop: loop {
        'label_loop2: loop{
            break 'label_loop;
        }
        
    }

    // for loop example
    for i in 0..3{
        println!("{}",i);
    }

    for i in 0..=3{
        println!("{}",i);
    }   

    let numbers: [i32; 3] = [10,20,30];
    for n in numbers.iter(){
        println!("{}",n);
    }

    let numbers = [(1,3),(2,4)];
    for (i, j) in numbers.iter(){
        println!("{} {}",i, j);
    }

    // for loop string
    let x = "hello";
    for i in x.chars(){
        println!("{}",i);
    }

    //string
    let x = String::from("Hello");
    let x = "Hello".to_string();

    println!("{}",x);

    //Collection
    let mut x = Vec::new();
    x.push(10);
    x.push(20);

    println!("{:#?}",x);

    //Hashmap
    let mut x: HashMap<&str, &str> = HashMap::new();
    x.insert("TH","Thailand");
    x.insert("US","United State");
    println!("{:?}",x);
    let y: Option<&&str> = x.get("TH");
    println!("{}",y.unwrap());

    //Struct
    let p = Person::new("Bond".to_string(), 18);
    p.hello();

    //Traits
    p.speak();

    //Enum
    let x: Colors = Colors::Blue;


    match x {
        Colors::Red => println!("red"),
        Colors::Green => println!("green"),
        _ => println!("blue"),
    }

    //////// 
    let mut color= "";
    match x {
        Colors::Red => color = "red",
        Colors::Green => color = "green",
        _ => color = "blue",
    }
    println!("enum basic type 2 {}",color);

    /////
    let color2 = match x {
        Colors::Red => "red",
        Colors::Green => "green",
        Colors::Blue => "blue",
    };
    println!("enum basic type 3 {}",color2);
    
    let x = check_grade(-1);
    match x {
        GradeResult::Error(e) => println!("{}", e),
        GradeResult::Value(grade) => println!("{}", grade),
    }

    let x = check_grade(80);
    match x {
        GradeResult::Error(e) => println!("{}", e),
        GradeResult::Value(grade) => println!("{}", grade),
    }

}


fn check_grade(score: i32) -> GradeResult {
    if score < 0 || score > 100 {
        return GradeResult::Error("Score is not collect".to_string());
    }
    return GradeResult::Value("your grade is A".to_string());
}

enum GradeResult {
    Value(String),
    Error(String),
}

enum Colors {
    Red,
    Green,
    Blue,
}

fn get_number() -> i32 {
    let a = 10;
    let b = 20;
    a+b // if last line mean return
}

