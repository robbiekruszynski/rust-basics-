//Primitive str = Immutable fixed-length string somewhere in memory
//String = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run() {
    let mut hello = String::from("Hello");

    // Get length
    println!("Length: {}", hello.len());

    hello.push('W');
    //adds a single char to the string

    hello.push_str("orld");
    //adds more than a char to the string

    //capacity in bytes
    println!("Capacity: {}", hello.capacity());
    //check if empty
    println!("Is Empty: {}", hello.is_empty());
    //check if string holds something
    println!("Contains 'World' {}", hello.contains("World"));
    //replace
    println!("Reaplce: {}", hello.replace("World", "There"));

    //loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    //create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    //Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", s);
}
