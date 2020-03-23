//Arrays - Fixed list where elements are the same data types

// when it comes to arrays the data type and length have to be 100%

//with arrays you can mutate the values but you can not change the length

//you can add use std:mem and just run mem below
use std::mem;
pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    //re-assign a value
    numbers[2] = 20;

    println!("{:?}", numbers);
    //Get Single value
    println!("Single value: {}", numbers[0]);

    //get array length
    println!("Array length: {}", numbers.len());

    //arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    //Get slice
    let slice: &[i32] = &numbers;

    println!("Slice: {:?}", slice);

    //Slice target will only print out 
    let slice: &[i32] = &numbers[0..2];

    println!("Slice: {:?}", slice);
}
