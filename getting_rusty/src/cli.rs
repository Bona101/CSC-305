use std::env;

pub fn run() {
    let v: Vec<String> = env::args().collect();
    let command = v[1].clone();

    if command == "password" {
        println!("Welcome");
    } else {
        println!("Vector: {:?}", v);
    }


}