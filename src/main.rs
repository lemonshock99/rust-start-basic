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
    speaking::Speaking,
    genpassword::gen_password,
    getip::getip,
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


    //Example call function in module
    gen_password();

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

    // Enum default in Rust Option
    let x = check_grade2(-1);
    match x {
        None => println!("Error"),
        Some(v) => println!("{}", v) ,
    }


    // Enum default in Rust Result

    let x = check_grade3(109);
    match x {
        Err(e) => println!("{}", e),
        Ok(v) => println!("{}", v) ,
    }

    // if Result sure not Error you can
    // ------- 01 
    let x = check_grade3(100);
    if x.is_err() {
        return;
    }
    let y = x.unwrap();

    // ------- 02
    let x = check_grade3(100);
    if let Ok(v) = x {
        println!("{}", v)
    }
    
    // ------- 03
    let x = check_grade3(100);
    let y = match x {
        Err(e) => {
            println!("{}",e);
            return;
        }
        Ok(v) => v,
    };


    // CLosures ---------- Annonymous Function
    let x = |a,b| a + b;
    let y = x(10, 20);
    println!("Closures Demo a+b = {}", y);

    // How to mix closures with function
    let y = cal(10, 20, x);
    println!("Function with Closures 1 {}", y);

    let y = cal(10, 20, |a,b| a + b);
    println!("Function with Closures 2 {}", y);

    let y = cal(10, 20, add);
    println!("Function with Closures 3 {}", y);

    getip();

    // Ownership and Borrow ----- ***specific Heap memory
    println!("====================================================");


    let crabby_master = "Crabby Master".to_string();

    // =====================
    // let crabby_01 = crabby_master;
    // println!("{}",crabby_01);
    // println!("{}",crabby_master); // error because crabby_01 got Ownership

    let crabby_02 = crabby_master.clone();
    println!("{} was Clone by Crabby02",crabby_02);
    println!("I'm {} OG",crabby_master);


    let crabby_03 = &crabby_master;
    println!("{} was Borrowing by Crabby03",crabby_03);
    println!("I'm {} OG",crabby_master);
    
    
    // =====================

    let mut crabby_master_mut = "Crabby Master mutable".to_string();
    let crabby_04 = &mut crabby_master_mut;

    // println!("{} was Borrowing by Crabby04",crabby_04);
    // println!("I'm {}",crabby_master_mut);

    *crabby_04 = "Crabby High Grand Master".to_string();
    println!("Crabby04 change value to {}",crabby_04);
    // println!("I'm {}",crabby_master_mut); // error bacause Ownership transfer to Crabby 04

    crabby_04.push_str(" Elite");
    println!("I'm {} ... crabby04", crabby_04);

    println!("I'm Original {}",crabby_master_mut); // can execute bacause Crabby 04 finish ... memory was destroyed


    
    // =====================




}


fn cal<F: Fn(i32, i32) -> i32> (a: i32, b: i32, f:F) -> i32 {
    f(a,b)
}

fn cal2<F> (a: i32, b: i32, f:F) -> i32 
where 
    F: Fn(i32, i32) -> i32,
{
    f(a,b)
}

fn add(a:i32, b:i32) -> i32 {
    a + b
}

fn check_grade(score: i32) -> GradeResult {
    if score < 0 || score > 100 {
        return GradeResult::Error("Score is not collect".to_string());
    }
    return GradeResult::Value("your grade is A".to_string());
}

fn check_grade2(score: i32) -> Option<String> {
    if score < 0 || score > 100 {
        return None;
    }
    return Some("A".to_string());
}


fn check_grade3(score: i32) -> Result<String, String> {
    if score < 0 || score > 100 {
        return Err("Score is not correct".to_string());
    }
    Ok("A".to_string())
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

