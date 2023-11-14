// Variables hold prinitve data or references to data
// Variables in Rust are immutabe by default
// Rust is a block-scoped language

pub fn run () {
    let mut name: &str = "Bona";
    println!("My name is {}", name);

    let bon: bool = true;

    name = "meytfrt";

    println!("My name is {}", name);

    // Define Constants
    const UDA: i32 = 0091;

    println!("{}", UDA);

    // Assignment to multiple variables at once
    let (bo, co) = (9, "k");

    print!("{{}}{}{}{}", bo, co, bon);
    
}