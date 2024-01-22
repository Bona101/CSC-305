pub fn run () {
    // Primtive Types
    // Integers: u8, i8, u16, i16, u32, u64, i64, u128, i128 (number of bits they can take in memory)
    // Floats: f32, f64
    // Boolean (bool)
    // Characters (char)
    // Tuples 
    // Arrays


    // Rust is a statically typed language, which means that it must know the types of all variables at compile time,
    // however, the compiler can usually infer what type we want to use based on the value and how we use it.

    let emoji = '\u{1F600}';

    let which = 5 < 4;

    println!("{:?}", (emoji, which));
}