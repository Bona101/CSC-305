#![allow(dead_code)]


use std::io;

// mod print;
// mod variables;
// mod types;
// mod variables;
// mod strings;
// use strings::*;
// mod arrays;
// mod vectors;
// mod conditionals;
// mod loops;
// mod functions;
// mod pointer_ref;
// mod structs;

// mod greetings;

// use greetings::{spanish::default_greeting, french, english};

// extern crate hello_world_lib;


    // println!("PRIMITIVES: SCALARS");
    // scalar::scalars::run();
    // println!("\nPRIMITIVES: COMPOUNDS: ARRAYS_AND_SLICES");
    // compound::arrays_and_slices::run();
    // println!("\nPRIMITIVES: COMPOUNDS: TUPLES");
    // compound::tuples::run();
    // println!("\nDERIVED: USER_DEFINED: STRUCTS");
    // user_defined::structs::run();
    // println!("\nDERIVED: USER_DEFINED: ENUMS");
    // user_defined::enums::run();
    // println!("\nDERIVED: USER_DEFINED: UNIONS");
    // user_defined::unions::run();
    // println!("\nDERIVED: FUNCTIONS: CLOSURES");
    // functions::closures::run();

    // let mut s = String::new();
    // s = String::from("45");
    // let a = io::stdin();
    // loop {
    //     if s == "458" {
    //         break;
    //     }
    //     println!("{}", s);
    // }
    // user_defined::structs::runn();
    // user_defined::all_user_defined::run2();
    // functions::everything_in_closures::run3();












// Ownership Rules
// 1. Each value in Rust has a variable that's called its owner
// 2. There can only be one owner at a time
// 3. When the owner goes out of scope, the value will be dropped

// Reference Rules 
// 1. At any given time, you can have either only one mutable reference (and no immutable reference) or any number of immutable references
// 2. References must be valid