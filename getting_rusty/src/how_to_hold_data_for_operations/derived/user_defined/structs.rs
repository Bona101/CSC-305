// An attribute to hide warnings for unused code.
#![allow(dead_code)]
use std::fmt;

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
#[derive(Debug)]
struct Unit;

#[derive(Debug)]
struct Any<T>(T);

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

struct Structure(i32);

// To use the `{}` marker, the trait `fmt::Display` must be implemented
// manually for the type.
    impl fmt::Display for Structure {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", self.0)
    }

}


#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "( {} {} )\n( {} {} )", self.0, self.1, self.2, self.3)
    }
}

fn transpose(matrix: Matrix) -> String {
    format!("( {} {} )\n( {} {} )", matrix.0, matrix.2, matrix.1, matrix.3)
}

#[derive(Debug)]
struct Rect {
    length: i32,
    width: i32,
    name: &'static str
}

impl From <&'static str> for Rect {
    fn from(s: &'static str) -> Self {
        let mut parts = s.split(',');
        let length = match parts.next() {
            Some(val) => val.parse::<i32>().unwrap(),
            None => 0
        };
        let width = match parts.next() {
            Some(val) => val.parse::<i32>().unwrap(),
            None => 0
        };
        let name = match parts.next() {
            Some(val) => val,
            None => ""
        };

        Rect {
            length,
            width,
            name
        }
    }
}

impl PartialEq for Rect {  // So apparently the PartialEq allows you do this -> rect1 == rect2
    fn eq(&self, other: &Self) -> bool {
        self.length == other.length && self.width == other.width
    }
}

struct Triangle {
    side1: i32,
    side2: i32,
    side3: i32,
    name: &'static str,
}


pub fn runn() {
    let rect = Rect::from("1,5,name");
    let rect2 = Rect::from("10,5,ne");
    
    println!("{:?}", rect);
    let t = rect.eq(&rect2);
    println!("t = {}", t);
    if (rect == rect2){
        print!("qwe");
    }
}
pub fn run() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    // Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 5.2, ..point };

    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point { x: left_edge, y: top_edge } = point;
    println!("ip{}, {}", left_edge, top_edge);
    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    // Instantiate a unit struct
    let _unit = Unit;
    println!("un{:?}", _unit);

    let strr = Any("i");
    let num = Any(232);
    println!("{:?}", num);
    println!("{:?}", strr);

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    let q = Structure(45);
    println!("{}", q);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("Matrix implementing std::fmt::Display: {}", matrix);
    println!("Matrix printed using debug trait: {:?}", matrix);
    println!("Transposed Matrix implementing std::fmt::Display: {}", transpose(matrix));
}