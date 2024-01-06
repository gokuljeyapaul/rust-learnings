mod person;
mod gender;

use person::Person;
use gender::Gender;

fn main() {
    let ironman = Person {   
        name: "Iron Man".to_string(),
        age: 32,
        gender: Gender::MALE
    };

    // Print the struct
    println!("{:?}", ironman);
    // Or pretty-print the struct
    println!("{:#?}", ironman);
}
