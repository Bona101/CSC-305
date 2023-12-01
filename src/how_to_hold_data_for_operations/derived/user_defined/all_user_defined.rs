#![allow(dead_code)] //This suppresses warnings when a given declared function is  not used.

use core::cmp::Ordering;
use std::fmt::format; //Used for comparison of value sizes 
use std::num;

pub enum Comp { //Enumerate Comparison
    LessThan,
    GreaterThan,
    Equal,
}

pub enum Gender { //Enumerate Gender
    Male,
    Female,
}

#[derive(Debug)] //Decorate the struct Person. Debug is an inbuilt trait. This statement will provoke impl Debug for Person; Metaprogramming
struct Person {
    name: String,
    age: u8,
}
struct Unit;
// A unit struct
//Have no state of their own but useful for
//implementing some trait.
//E.g. struct Global is a unit struct that can implement traits like Allocator
//std::fmt::Error is a unit struct that implements
//traits like Error

//A tuple struct
struct Pair(i32, f32); //No named fields but has fields

//A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct. Below Point is used as datatype in Rectangle
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

pub fn run() {

    //declare a variable of type Person and assign values.
    let person = Person {
        name: String::from("Peter"),
        age: 27,
    };
    println!("{:#?}", person); //{:#?} implies pretty debug print person. :? is for debug print and :#? is for pretty debug print

    // Instantiate a unit struct
    let _unit = Unit;//As simple as that. If Unit implements some trait, then _unit will demand those implementations

    //declare a Point
    let point = Point { x: 10.3, y: 0.4 };

    //Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 5.2, ..point };
    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a let binding.
    let Point {
        x: left_edge,//left_edge here will be declared. If you use x:f32 only, x will be declared.
        y: top_edge,//top_edge here will be declared. If you use y:f32 only, y will be declared.
    } = point;
    dbg!(&left_edge,&top_edge);


    let _rectangle = Rectangle { //I used _ with rectangle to silence warning knowing that the variable is not used.
        //struct instantiation is an expression too as used below in Point
        top_left: Point {
            x: left_edge,//left_edge is available, thanks to the destructuring above
            y: top_edge,//top_edge is available, thanks to the destructuring above
        },
        bottom_right,
    };

    //Instantiate a tuple struct
    let pair = Pair(1, 0.1);
    //Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    //Destructure a tuple struct
    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);
}

pub fn run2() {
    let rectangle1 = Rect::default();
    
    println!("{}", rectangle1.length);
    println!("{}", rectangle1.width);
    println!("{}", rectangle1.name);

    let rectangle2 = Rect::new(1.0, 3.0, "Rectangle2");
    let rectangle3 = Rect::from("4,5,Rectangle3");

    //Compare using PartialOrd
    let result1 = rectangle1.partial_cmp(&rectangle2);
    println!("result1 = {:?}", result1);
    println!("result1 = {:?}", result1.unwrap());

    let result2 = rectangle1.le(&rectangle2);
    println!("result2 = {:?}", result2);

    //Compare using PartialEq
    let result3 = rectangle2.eq(&rectangle3);
    println!("result3 = {:?}", result3);

    let result4 = rectangle2.ne(&rectangle3);
    println!("result4 = {:?}", result4);
}

//Exercise
/*
I need similar implementation for Circle and Triangle
Besides Area, I need Perimeter and comparison on the basis of Perimeter
In your submission, I need a comment against every line of code about what it is mearnt to achieve
 */


 //Let's work on user-defined traits. Traits enable us achieve polymorphism.
//We are designing Shape below for the purpose of 
//specifying all expected functions and methods in any struct that implements Shape.

// The trait below shows the functions all shapes have in common
trait Shape {  // declaration of Shape trait
    fn default() -> Self;  // for creating default shapes
    fn area(&self) -> f32; // for calculating area
    fn perimeter(&self) -> f32; // for calculating perimeter
    fn set_name(&mut self, name: &'static str); // for setting the name of the shape
    fn get_name(&self) -> &str; // for getting the name of the shape
} // end of shape declaration
//The use of 'static lifetime above ensures that our
//compiler is clear about the availability of those values, as they are borrowed.
//static will be available throughout the lifetime of the program.

#[derive(Debug, Clone)] // we are using the derive attribute to implemnt the Debug and Clone traits for our struct Triangle 
struct Triangle {  // declaration of triangle struct
    left_side: f32,  // left side of the triangle 
    right_side: f32,  // right side of the triangle 
    bottom_side: f32, // bottom side of the triangle 
    name: &'static str, // name of the triangle
} // end of triangle declaration

// implementation of triangle
impl Triangle {
    // Define a associated function named new to create a new instance of Triangle
    fn new(left_side: f32, right_side: f32, bottom_side: f32, name: &'static str) -> Self {
        // Return a new Triangle instance with specified sides and name
        Triangle { 
            left_side, // left_side argument is assigned to the left side of the triangle
            right_side,  // right_side argument is assigned to the right side of the triangle
            bottom_side, // bottom_side argument is assigned to the bottom side of the triangle
            name, // name argument is assigned to the name of the triangle
        }
    }

    // Define a method that changes the value of the left side
    fn set_left_side(&mut self, left_side: f32) {
        // Set the value of the left side to the specified value
        self.left_side = left_side;
    }

    // Define a method that gets the value of the left side
    fn get_left_side(&self) -> f32 {
        // Return the value of the left side
        self.left_side
    }

    // Define a method that changes the value of the right side
    fn set_right_side(&mut self, right_side: f32) {
        // Set the value of the right side to the specified value
        self.right_side = right_side;
    }

    // Define a method that gets the value of the right side
    fn get_right_side(&self) -> f32 {
        // Return the value of the right side
        self.right_side
    }

    // Define a method that changes the value of the bottom side
    fn set_bottom_side(&mut self, bottom_side: f32) {
        // Set the value of the bottom side to the specified value
        self.bottom_side = bottom_side;
    }

    // Define a method that gets the value of the bottom side
    fn get_bottom_side(&self) -> f32 {
        // Return the value of the bottom side
        self.bottom_side
    }

    // Define a method to calculate the half of the perimeter
    fn get_s(&self) -> f32 {
        // Calculate and return the semi-perimeter using the formula 0.5 * (a + b + c)
        0.5 * (self.left_side + self.right_side + self.bottom_side)
    }
}

// Define implementation of the Shape trait for the Triangle struct
impl Shape for Triangle {
    // Define a default method for creating a default Triangle instance
    fn default() -> Self {
        Triangle { 
            left_side: 1.0, 
            right_side: 1.0, 
            bottom_side: 1.0, 
            name: "default_name",
        }
    }

    // Define a method to calculate the area of the triangle
    fn area(&self) -> f32 {
        let s = self.get_s(); //getting the semi-perimeter
        let area = f32::sqrt(s * (s - self.left_side) * (s - self.right_side) * (s - self.bottom_side)); // calculating the area of the triangle
        area // returning the area
    }

    // Define a method to calculate the perimeter of the triangle
    fn perimeter(&self) -> f32 {
        self.left_side + self.right_side + self.bottom_side // calculating and returning the perimeter of the triangle
    } 

    // Define a method to set the name of the triangle
    fn set_name(&mut self, name: &'static str) {
        self.name = name; // setting the name of the triangle
    }

    // Define a method to get the name of the triangle
    fn get_name(&self) -> &str {
        self.name // returning the name of the triangle
    }
}

// Implement the PartialEq trait for Triangle, comparing triangles based on area
impl PartialEq for Triangle {
    fn eq(&self, other: &Self) -> bool {
        self.perimeter() == other.perimeter() // check if the perimeter of the two are equal
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other) // check if the perimeter of the two are unequal
    }
}

// Implement the PartialOrd trait for Triangle, comparing triangles based on perimeter
impl PartialOrd for Triangle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.perimeter().partial_cmp(&other.perimeter())
    }
    // Provided methods
    //fn lt(&self, other: &Rhs) -> bool { ... }
    //fn le(&self, other: &Rhs) -> bool { ... }
    //fn gt(&self, other: &Rhs) -> bool { ... }
    //fn ge(&self, other: &Rhs) -> bool { ... }
}

// we are using the derive attribute to implemnt the Debug and Clone traits for our struct Circle 
#[derive(Debug, Clone)]
// Define a new struct Circle with radius and name properties
struct Circle {
    radius: f32, // radius property
    name: &'static str, // name property
}

impl Circle {
    // Define a method to create a new Circle instance
    fn new(radius: f32, name: &'static str) -> Self {
        // returning a new Circle
        Circle { 
            radius, // setting the radius argument of the function to the radius property
            name,
        }
    }

    // Define a method to set the radius of the circle
    fn set_radius(&mut self, radius: f32) {
        self.radius = radius;
    }

    // Define a method to get the radius of the circle
    fn get_radius(&self) -> f32 {
        self.radius // 
    }
}

// Implement the Shape trait for Circle
impl Shape for Circle {
    // Default method to create a default Circle instance
    fn default() -> Self {
        Circle {
            radius: 1.0,
            name: "default_name",
        }
    }
    
    // Associated function used to create a new Shape
    fn area(&self) -> f32 {
        3.14159 * f32::powf(self.radius, 2.0) // Calculate and return the area of the circle
    }

    // Method to calculate and return the perimeter of the circle
    fn perimeter(&self) -> f32 {
        2.0 * 3.14159 * self.radius
    }

    // Method to set the name of the circle
    fn set_name(&mut self, name: &'static str) {
        self.name = name;
    }

    // Method to get the name of the circle
    fn get_name(&self) -> &str {
        self.name
    }
}

// Implement the PartialEq trait for Circle
impl PartialEq for Circle {
    // Method to check if two circles are equal based on their perimeters
    fn eq(&self, other: &Self) -> bool {
        self.perimeter() == other.perimeter()
    }

    // Method to check if two circles are not equal based on their perimeters
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

// Implement the PartialOrd trait for Circle, comparing circles based on perimeter
impl PartialOrd for Circle {
    // Method to partially compare two circles based on their perimeters
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.perimeter().partial_cmp(&other.perimeter())
    }

   // Provided methods
    //fn lt(&self, other: &Rhs) -> bool { ... }
    //fn le(&self, other: &Rhs) -> bool { ... }
    //fn gt(&self, other: &Rhs) -> bool { ... }
    //fn ge(&self, other: &Rhs) -> bool { ... }
}

// Use Default to specify the availability of default instance creation without values passed for property
// But to avoid repetition it is specified in the declaration of trait Shape
#[derive(Debug, Clone)]
struct Rect { // defining a rectangle struct 
    length: f32, // defining the length
    width: f32, // defining the width
    name: &'static str, // defining the name
}

impl Rect { // implementation of the rectangle struct
    fn new(length: f32, width: f32, name: &'static str) -> Self {
        Rect {
            length,
            width,
            name,
        }
    }

    fn set_length(&mut self, length: f32) {
        self.length = length;
    }

    fn get_length(&self) -> f32 {
        self.length
    }

    fn set_width(&mut self, width: f32) {
        self.width = width;
    }

    fn get_width(&self) -> f32 {
        self.width
    }
}

impl Shape for Rect {
    //default default() function. Will override derived default if any. 
    fn default() -> Self {
        Rect {
            length: 1.0,
            width: 1.0,
            name: "default_name",
        }
    }
    
    //Associated function used to create a new Shape
    fn area(&self) -> f32 {
        self.length * self.width
    }

    fn perimeter(&self) -> f32 {
        self.length * 2.0 + self.width * 2.0
    }

    fn set_name(&mut self, name: &'static str) {
        self.name = name;
    }

    fn get_name(&self) -> &str {
        self.name
    }
}

//implement Partial Eq
impl PartialEq for Rect {
    fn eq(&self, other: &Self) -> bool {
        self.area() == other.area()
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialOrd for Rect {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.perimeter().partial_cmp(&other.perimeter())
    }
    // Provided methods
    //fn lt(&self, other: &Rhs) -> bool { ... }
    //fn le(&self, other: &Rhs) -> bool { ... }
    //fn gt(&self, other: &Rhs) -> bool { ... }
    //fn ge(&self, other: &Rhs) -> bool { ... }
}


//A conversion implementation into String
//Expects a string slice with length, width, name, separated by commas
impl From<&'static str> for Rect {
    fn from(s: &'static str) -> Rect {
        let mut parts = s.split(',');
        let length = match parts.next() {
            Some(val) => val.parse::<f32>().unwrap(),
            None => 0.0,
        };
        let width = match parts.next() {
            Some(val) => val.parse::<f32>().unwrap(),
            None => 0.0,
        };
        let name = match parts.next() {
            Some(val) => val,
            None => "",
        };
        Rect { length, width, name: &name }
    }
}

impl Into<String> for Rect { // implementing Into String for Rect to convert something related to Rect to a string
    fn into(self) -> String { // into method to return a string containing the name of the rectangle and the area.
        // Let's return a template literal
        format!("My name is {} and my area is {}", self.name, self.area())
    }
}

impl Into<Circle> for Rect { // implementing Into Circle for Rect to convert the Rect to a Circle of equal area
    fn into(self) -> Circle {  // into method to convert the rectangle to a circle of equal area
        Circle { // returning an equivalent circle
            radius: f32::sqrt(self.area() / 3.14159), // calculating the resultant radius and assigning it to the radius member of the struct Circle
            name: self.name, // the resultant circle takes the name of the rectangle
        }
    }
}

impl Into<Triangle> for Rect { // implementing Into Triangle for Rect to convert the Rect to a Triangle of equal area
    fn into(self) -> Triangle { // into method to convert the rectangle to a triangle of equal area
        let right_and_bottom = f32::sqrt(f32::powf(self.length, 2.0) + f32::powf(self.width, 2.0)); // calculating the length of the right and bottom sides
        Triangle {  // returning an equivalent triangle
            left_side: self.length * 2.0, // setting the left side of the triangle to twice the length of the rectangle
            right_side: right_and_bottom, // setting the right side of the triangle to the calculated value of right and bottom side
            bottom_side: right_and_bottom, // setting the right side of the triangle to the calculated value of right and bottom side
            name: self.name, // setting the name of the triangle to the name of the rectangle
        }
    }
}


pub fn excercise() {
    let recto = Rect::default();
    let circle: Circle = recto.clone().into();
    let triangle: Triangle = recto.clone().into();

    println!("{:?}", circle);
    println!("{:?}", triangle);

    let circle2 = Circle::new(2.0, "rand");
    let triangle2 = Triangle::new(3.0, 4.0, 5.0, "something");

    let cmp_circles = circle2.partial_cmp(&circle);
    let cmp_triangles = triangle2.partial_cmp(&triangle);

    println!("{:?}", cmp_circles);
    println!("{:?}", cmp_triangles);
    println!("circle2 is {:?} in terms of perimeter", cmp_circles.unwrap());
    println!("triangle2 is {:?} in terms of perimeter", cmp_triangles.unwrap());

    println!("{}", circle.eq(&circle2));
    println!("{}", triangle.eq(&triangle2));
}