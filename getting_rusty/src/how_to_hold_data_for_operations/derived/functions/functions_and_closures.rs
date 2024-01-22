
///Functions and Closures
//We have been using functions already, including the main() which is the program entry point
//In this section, we are particularly highlighting the fact that functions
//have a type unto themselves and variables of a given function type
//can be declared and passed to another function.
//So, we can have a series of function calls, the output of one becoming the input of
//the next. Herein lies the concept of higher order functions

//As already mentioned, in Rust, functions have their own types.
//Below is an illustration

///Function to add to two signed integers. Returns a signed integer
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

//The function type embodied in the above is fn(i32, i32) -> i32.
//Function type is defined by the keyword fn followed 
//by the optional expected parameter types
//and then the optional expected return type.

///Here we define a function name apply that is expected to receive the function type
/// above name f here, along with two other unsigned interger parameters named x and y
/// respectively
pub fn apply_add(f: fn(i32, i32) -> i32, x: i32, y: i32) -> i32 {
    f(x, y) //a call to the function passed, which in its turn is passed two other parameters
}


pub fn run() {
    let f = add;
    let x = 7;
    let y = 8;
    let z = apply_add(f, x, y);
    println!(
        "The result of applying add function f to {} and {} is {}",
        x, y, z
    );

}

///let's define another function that handles straight line graph formula
///Assuminng that m, c and x have to be passed.
///Here you can use a normal function.
///Below, we have to use array slice as x, otherwise, we will need to specify a size.
pub fn straight_line_function(m: i32, c: i32, xses: &[i32]) -> Vec<(i32, i32)> {
    let mut output: Vec<(i32, i32)> = Vec::new(); //you could also use vec![] to bring in initial arguments
    for x in xses {
        let y = (m * x) + c;
        output.push((*x, y)) //here we have to dereference the borrowed x, to get the value
    }
    output
    
}


pub fn run2() {
    let c = 10;
    let m = 20;
    let xses = [1, 2, 3, 4, 5];

    //Let's use our straight_line function above. We must pass m,c and xses as arguments
    let output = straight_line_function(m, c, &xses);
    println!("Points for straight line plot are {:?}", output);
}

pub fn run3() {
    run();
    run2();
}