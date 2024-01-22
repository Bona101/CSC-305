use core::cmp::Ordering;
use std::fmt::format; //Used for comparison of value sizes
use std::num;

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
trait Shape {
    // declaration of Shape trait
    fn default() -> Self; // for creating default shapes
    fn area(&self) -> f32; // for calculating area
    fn perimeter(&self) -> f32; // for calculating perimeter
    fn set_name(&mut self, name: &'static str); // for setting the name of the shape
    fn get_name(&self) -> &str; // for getting the name of the shape
} // end of shape declaration
  //The use of 'static lifetime above ensures that our
  //compiler is clear about the availability of those values, as they are borrowed.
  //static will be available throughout the lifetime of the program.

#[derive(Debug, Clone)] // we are using the derive attribute to implement the Debug and Clone traits for our struct Triangle
struct Triangle {
    // declaration of triangle struct
    left_side: f32,     // left side of the triangle
    right_side: f32,    // right side of the triangle
    bottom_side: f32,   // bottom side of the triangle
    name: &'static str, // name of the triangle
} // end of triangle declaration

// implementation of triangle
impl Triangle {
    // Define a associated function named new to create a new instance of Triangle
    fn new(left_side: f32, right_side: f32, bottom_side: f32, name: &'static str) -> Self {
        // Return a new Triangle instance with specified sides and name
        Triangle {
            left_side,   // left_side argument is assigned to the left side of the triangle
            right_side,  // right_side argument is assigned to the right side of the triangle
            bottom_side, // bottom_side argument is assigned to the bottom side of the triangle
            name,        // name argument is assigned to the name of the triangle
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
            left_side: 1.0,       // setting the left side to 1 by default
            right_side: 1.0,      // setting the right side to 1 by default
            bottom_side: 1.0,     // setting the bottom side to 1 by default
            name: "default_name", // setting the default name
        }
    }

    // Define a method to calculate the area of the triangle
    fn area(&self) -> f32 {
        let s = self.get_s(); //getting the semi-perimeter
        let area =
            f32::sqrt(s * (s - self.left_side) * (s - self.right_side) * (s - self.bottom_side)); // calculating the area of the triangle
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
        // partial_cmp function to check whether the perimeter of the triangle is less than, greater than or equal to the perimeter of the other triangle
        self.perimeter().partial_cmp(&other.perimeter()) // comparing perimeters
    }
    // Provided methods
    //fn lt(&self, other: &Rhs) -> bool { ... }
    //fn le(&self, other: &Rhs) -> bool { ... }
    //fn gt(&self, other: &Rhs) -> bool { ... }
    //fn ge(&self, other: &Rhs) -> bool { ... }
}

// we are using the derive attribute to implement the Debug and Clone traits for our struct Circle
#[derive(Debug, Clone)]
// Define a new struct Circle with radius and name properties
struct Circle {
    radius: f32,        // radius property
    name: &'static str, // name property
}

impl Circle {
    // Define a method to create a new Circle instance
    fn new(radius: f32, name: &'static str) -> Self {
        // returning a new Circle
        Circle {
            radius, // setting the radius argument of the function to the radius property
            name,   // setting the name argument of the function to the name property
        }
    }

    // Define a method to set the radius of the circle
    fn set_radius(&mut self, radius: f32) {
        self.radius = radius;
    }

    // Define a method to get the radius of the circle
    fn get_radius(&self) -> f32 {
        self.radius // returning the radius of the circle
    }
}

// Implement the Shape trait for Circle
impl Shape for Circle {
    // Default method to create a default Circle instance
    fn default() -> Self {
        Circle {
            // returning a default circle
            radius: 1.0,          // setting the radius to 1
            name: "default_name", // setting the name to 'default_name'
        }
    }

    // Method to calculate the area of the circle
    fn area(&self) -> f32 {
        3.14159 * f32::powf(self.radius, 2.0) // Calculate and return the area of the circle
    }

    // Method to calculate and return the perimeter(circumference) of the circle
    fn perimeter(&self) -> f32 {
        2.0 * 3.14159 * self.radius // returning the perimeter
    }

    // Method to set the name of the circle
    fn set_name(&mut self, name: &'static str) {
        self.name = name; // setting name to new name
    }

    // Method to get the name of the circle
    fn get_name(&self) -> &str {
        self.name // returning the name of the circle
    }
}

// Implement the PartialEq trait for Circle
impl PartialEq for Circle {
    // Method to check if two circles are equal based on their perimeters
    fn eq(&self, other: &Self) -> bool {
        self.perimeter() == other.perimeter() // checking if the two circles are equal based on perimeter
    }

    // Method to check if two circles are not equal based on their perimeters
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other) // checking if the two circles are unequal based on perimeter
    }
}

// Implement the PartialOrd trait for Circle, comparing circles based on perimeter
impl PartialOrd for Circle {
    // Method to partially compare two circles based on their perimeters
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.perimeter().partial_cmp(&other.perimeter()) // checking whether the perimeter of the circle is less than, greater than or equal to the perimeter of the other circle
    }

    // Provided methods
    //fn lt(&self, other: &Rhs) -> bool { ... }
    //fn le(&self, other: &Rhs) -> bool { ... }
    //fn gt(&self, other: &Rhs) -> bool { ... }
    //fn ge(&self, other: &Rhs) -> bool { ... }
}

// Use Default to specify the availability of default instance creation without values passed for property
// But to avoid repetition it is specified in the declaration of trait Shape
#[derive(Debug, Clone)] // // we are using the derive attribute to implement the Debug and Clone traits for our struct Rect
struct Rect {
    // defining a rectangle struct
    length: f32,        // defining the length
    width: f32,         // defining the width
    name: &'static str, // defining the name
}

impl Rect {
    // implementation of the rectangle struct

    // Method to create a new rectangle
    fn new(length: f32, width: f32, name: &'static str) -> Self {
        Rect {
            // returning the rectangle
            length, // setting the length to the passed length argument
            width,  // setting the width to the passed width argument
            name,   // setting the name to the passed name argument
        }
    }

    // Method to set the length of the rectangle
    fn set_length(&mut self, length: f32) {
        self.length = length; // changing the length of the rectangle to the passed length argument
    }

    // Method to get the length of the rectangle
    fn get_length(&self) -> f32 {
        self.length // returning the length of the rectangle
    }

    // Method to set the width of the rectangle
    fn set_width(&mut self, width: f32) {
        self.width = width; // changing the width of the rectangle to the passed width argument
    }

    // Method to get the width of the rectangle
    fn get_width(&self) -> f32 {
        self.width // returning the width of the rectangle
    }
}

impl Shape for Rect {
    // implementation of the Shape trait for the Rect struct

    // Default method to create a default Rect instance
    fn default() -> Self {
        Rect {
            // returning the rect
            length: 1.0,          // setting default length to 1
            width: 1.0,           // setting default width to 1
            name: "default_name", // setting default name to 'default_name'
        }
    }

    // Associated function used to create a new Shape
    fn area(&self) -> f32 {
        self.length * self.width // Calculate and return the area of the rectangle
    }

    // Method to calculate and return the perimeter of the rectangle
    fn perimeter(&self) -> f32 {
        self.length * 2.0 + self.width * 2.0 // Calculate and return the perimeter of the rectangle
    }

    // Method to set the name of the rectangle
    fn set_name(&mut self, name: &'static str) {
        self.name = name; // changing the name of the rect to the passed name argument
    }

    // Method to get the name of the rectangle
    fn get_name(&self) -> &str {
        self.name // returning the name of the rect
    }
}

// Implement Partial Eq for Rect
impl PartialEq for Rect {
    // Method to check if two rectangles are equal based on their areas
    fn eq(&self, other: &Self) -> bool {
        self.area() == other.area() // checking if the area of the two rects are equal
    }

    // Method to check if two rectangles are not equal based on their areas
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other) // checking if the area of the two rects are unequal
    }
}

// Implement Partial Ord for Rect
impl PartialOrd for Rect {
    // Method to partially compare two rectangles based on their perimeters
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.perimeter().partial_cmp(&other.perimeter()) // checking whether the perimeter of the rect is less than, greater than or equal to the perimeter of the other rect
    }

    // Additional provided methods: lt, le, gt, ge
    // fn lt(&self, other: &Rhs) -> bool { ... }
    // fn le(&self, other: &Rhs) -> bool { ... }
    // fn gt(&self, other: &Rhs) -> bool { ... }
    // fn ge(&self, other: &Rhs) -> bool { ... }
}

//A conversion implementation from String
//Expects a string slice with length, width, name, separated by commas

impl From<&'static str> for Rect {
    // Implementation of the From trait for converting a string slice into a Rect
    fn from(s: &'static str) -> Rect {
        let mut parts = s.split(','); // Split the string into parts using commas

        // Extract the length from the first part and parse it into a f32, defaulting to 0.0 if not present
        let length = match parts.next() {
            Some(val) => val.parse::<f32>().unwrap(), // extracting the length from the first part and parse it into a f32
            None => 0.0, // length becomes 0.0 if first part not present
        };

        // Extract the width from the second part and parse it into a f32, defaulting to 0.0 if not present
        let width = match parts.next() {
            Some(val) => val.parse::<f32>().unwrap(), // extract the width from the second part and parse it into a f32
            None => 0.0, // width becomes 0.0 if second part not present
        };

        // Extract the name from the third part, defaulting to an empty string if not present
        let name = match parts.next() {
            Some(val) => val, // extract the name from the third part
            None => "",       // name becomes empty string if third part not present
        };

        // Create and return a new Rect instance with the extracted values
        Rect {
            length,
            width,
            name: &name,
        }
    }
}

impl Into<String> for Rect {
    // implementing Into String for Rect to convert something related to Rect to a string
    fn into(self) -> String {
        // into method to return a string containing the name of the rectangle and the area.
        // Let's return a template literal
        format!("My name is {} and my area is {}", self.name, self.area())
    }
}

impl Into<Circle> for Rect {
    // implementing Into Circle for Rect to convert the Rect to a Circle of equal area
    fn into(self) -> Circle {
        // into method to convert the rectangle to a circle of equal area
        Circle {
            // returning an equivalent circle
            radius: f32::sqrt(self.area() / 3.14159), // calculating the resultant radius and assigning it to the radius member of the struct Circle
            name: self.name, // the resultant circle takes the name of the rectangle
        }
    }
}

impl Into<Triangle> for Rect {
    // implementing Into Triangle for Rect to convert the Rect to a Triangle of equal area
    fn into(self) -> Triangle {
        // into method to convert the rectangle to a triangle of equal area
        let right_and_bottom = f32::sqrt(f32::powf(self.length, 2.0) + f32::powf(self.width, 2.0)); // calculating the length of the right and bottom sides
        Triangle {
            // returning an equivalent triangle
            left_side: self.length * 2.0, // setting the left side of the triangle to twice the length of the rectangle
            right_side: right_and_bottom, // setting the right side of the triangle to the calculated value of right and bottom side
            bottom_side: right_and_bottom, // setting the right side of the triangle to the calculated value of right and bottom side
            name: self.name, // setting the name of the triangle to the name of the rectangle
        }
    }
}

pub fn excercise() {
    let rect = Rect::default(); // creating a rectangle instance
    let circle: Circle = rect.clone().into(); // creating an equivalent circle for the above rectangle
    let triangle: Triangle = rect.clone().into(); // creating an equivalent triangle for the above rectangle
                                                  // I'm cloning so rect can still retain its value

    println!("Rectangle: {:?}", rect); // printing rect
    println!(
        "Circle equivalent in area to the above Rectangle: {:?}",
        circle
    ); // printing circle
    println!(
        "Triangle equivalent in area to the above two shapes {:?}",
        triangle
    ); // printing triangle

    let circle2 = Circle::new(2.0, "rand"); // creating a circle instance with radius 2 and name 'rand'
    let triangle2 = Triangle::new(3.0, 4.0, 5.0, "something"); // creating a triangle instance with left side 3, right side 4, bottom side 5, and name 'something'

    let cmp_circles = circle2.partial_cmp(&circle); // compare circle2 with circle to see which is greater or if they are equal
    let cmp_triangles = triangle2.partial_cmp(&triangle2); // compare triangle2 with triangle to see which is greater or if they are equal

    println!("{:?}", cmp_circles); // print the circles comparison value
    println!("{:?}", cmp_triangles); // print the triangles comparison value
    println!(
        "circle2 is {:?} in terms of perimeter",
        cmp_circles.unwrap()
    ); // print just the circles comparison value without the 'Some' keyword and the parentheses, together with the remaining specifed string.
    println!(
        "triangle2 is {:?} in terms of perimeter",
        cmp_triangles.unwrap()
    ); // print just the triangles comparison value without the 'Some' keyword and the parentheses, together with the remaining specifed string.

    println!("{}", circle.eq(&circle2)); // print a boolean indicating whether the two circles are equal
    println!("{}", triangle.eq(&triangle2)); // print a boolean indicating whether the two triangles are equal
}
