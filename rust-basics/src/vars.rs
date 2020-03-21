//Variables hold primitive data or references to data
//Variables are immutable by default
//Rust is a block-scoped language

pub fn run() {
    let name = "Percy";
    let mut age = "2";
    println!("His name is {} and he is {} years old", name, age);

    //define constant
    // usually uppercase and you do have to add a type
    const ID: i32 = 001;
    println!("ID: {}", ID);

    //assign multiple vars at once
    let (my_name, my_age) = ("Percy", 132);
    println!("{} is {}", my_name, my_age);
}
