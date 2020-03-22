//Arrays - Fixed list where elements are the same data types

// when it comes to arrays the data type and length have to be 100%

//with arrays you can mutate the values but you can not change the length
pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    //re-assign a value
    numbers[2] = 20;

    println!("{:?}", numbers);
    //Get Single value
    println!("Single value: {}", numbers[0]);
}
