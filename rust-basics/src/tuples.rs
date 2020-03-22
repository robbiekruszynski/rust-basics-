//Tuples are grouped together values of different types
//Max 12 elements

pub fn run() {
    let person: (&str, &str, i8) = ("Kate", "Goss", 38);
    println!(
        "{}{} is her name, {} is her age",
        person.0, person.1, person.2
    );
}
