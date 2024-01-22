struct Point {
    x: i32,
    y: i32,
}

#[repr(u8)]
enum ExampleEnum {
    VariantA = 99,
    VariantB,
    VariantC = 90,
}

pub fn run() {
    // Creating a Vec<i32> and initializing it with values
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    // Accessing elements
    println!("First element: {}", numbers[0]);

    // Iterating over elements
    for num in &numbers {
        println!("{}", num);
    }
    println!("num: {:?}", numbers);

    // Pushing a new element
    let mut more_numbers = Vec::new();
    more_numbers.push(6);
    more_numbers.push(7);

    // Combining two vectors
    let combined_vec = numbers.clone().into_iter();
    let m = combined_vec.clone().chain(more_numbers);
    let mo: Vec<i32> = m.clone().collect();

    // Printing the combined vector
    println!("{:?}", combined_vec);
    println!("{:?}", m);
    println!("{:?}", mo);
    println!("{:?}", numbers);
}

pub fn run_again() {
      

    if 0b1000 < 0b10 | 0b1001 {
        println!("{}", true);
    }

    let tup: (i32, u32, char, Vec<i32>, [i32; 4]) = (34, 4, 'k', vec![1,2,3], [4,5,6,23]);
 
    let mut vect = vec![2,4,5];
    let aar = [2,4,5];
    // aar.push(23);
    vect.push(23);
    println!("tup: {:?}", aar);

    // A heap-allocated array, coerced to a slice
    let mut boxed_array: [i32; 3] = [1, 2, 3];
    boxed_array = [35, 2, 3];
    let mut a = "db";
    a = &a[..];
    println!("{}", a.len());

    // A (shared) slice into an array
    let mut slice: &[i32] = &boxed_array[..];
    println!("{:?}", slice);

    slice = &slice[1..];
    println!("{:?}", slice);

    let mut mutable = 12; // Mutable `i32`
    mutable = 21;
    println!("{}", mutable);

    // Error! The type of a variable can't be changed.
    // mutable = true;

    // Variables can be overwritten with shadowing.
    let mut mutable = true;
    mutable = false;
    println!("{}", mutable);

    // Integer addition
    println!("1 + 2 = {}", 1u32 + 2);

    // Integer subtraction
    println!("1 - 2 = {}", 1i32 - 2);
    // TODO ^ Try changing `1i32` to `1u32` to see why the type is important

    // Scientific notation
    println!("1e4 is {}, -2.5e-3 is {}", 1e4, -2.5e-3);

    // Short-circuiting boolean logic
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // Use underscores to improve readability!
    println!("One million is written as {}", 1_000_000u32);


    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    let a = 9;
    let my_point = Point { x: 10, y: 20 };

    // Using println! with the Debug trait
    println!("{:?}", my_point);

    // Using dbg! macro for debugging
    dbg!(my_point);

    // extra::run();

    // println!("{}", run());

    // println!("{}", run2());

    // variables::run();

    // types::run();

    // run();

    // arrays::run();

    // vectors::run();

    // println!("{:?}", (default_greeting(), french::default_greeting(), english::greetings()));

    // conditionals::run();

    // loops::run();

    // functions::run();

    // pointer_ref::run();

    // structs::run();

    
    let point = Point { x: 10, y: 20 };

    match point {
        Point { x, y } => println!("x: {}, y: {}", x, y),
    }

    let a = ExampleEnum::VariantA;
    let b = ExampleEnum::VariantB;
    let c = ExampleEnum::VariantC;

    println!("VariantA: {}", a as i32 as f64);  // VariantA: 0
    println!("VariantB: {}", b as i16);  // VariantB: 1
    println!("VariantC: {}", c as u8);  // VariantC: 2

    let value = 42;

    match value {
        0 | 1 => println!("Zero or one"),
        2..=100 => println!("Between 2 and 100"),
        _ => println!("Some other value"),
    }

/////////////////////////////////////////////////////////////////////////////////////////////

    fn clo() -> Option<i32> {
        Some(None::<i32>? + 9)
    }

    fn clon() -> Option<i32> {
        Some(None::<i32>?)
    }

    fn clom() -> Option<i32> {
        Some(Some(2)? + 1)
    }

    fn clog() -> Option<i32> {
        Some(None?)
    }
    println!("{:?}", clo());
    println!("{:?}", clon());
    println!("{:?}", clom());
    println!("{:?}", clog());
}
