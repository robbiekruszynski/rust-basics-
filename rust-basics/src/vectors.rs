//Vectors are arrays you can manipulate, reziable arrays

use std::mem;
pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    //re-assign a value
    numbers[2] = 20;

    //add on to vector
    numbers.push(6);
    numbers.push(7);

    //Dump the last value
    numbers.pop();

    println!("{:?}", numbers);
    //Get Single value
    println!("Single value: {}", numbers[0]);

    //get vector length
    println!("Vector length: {}", numbers.len());

    //arrays are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    //Get slice
    let slice: &[i32] = &numbers;

    println!("Slice: {:?}", slice);

    //Slice target will only print out
    let slice: &[i32] = &numbers[0..2];

    println!("Slice: {:?}", slice);

    // Loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }
}
