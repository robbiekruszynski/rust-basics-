//Primtive Types --
// Integers: u8, i8, i16, u64, u128, i128 (number of bits they take in memory)
// Floats: f32, f64
// Boolean (bool)
// Characters (char)
// Tuples
// Arrays

//Rust is a statically typed languiage, which means that is must know the types
//of all variables at compile time, however, the compiler can usually infer what type we want to use based on the value and how we use it

pub fn run() {
    //default is "i32"
    let x = 1;
    // /default is "f64"
    let y = 2.5;

    //add explicit type
    let b: i64 = 4545454545;

    //find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    //boolean
    let is_active = true;

    //get boolean from expression
    let is_greater: bool = 2 > 5;

    //char is always a single character and has single quotes
    let a1 = 'a';
    println!("{:?}", (x, y, x, is_active, is_greater, a1));
}
