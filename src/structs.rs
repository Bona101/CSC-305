// Traditional Struct

// use std::fmt::format;

struct Color {
    red: u8,
    green: u8,
    blue: u8
}

impl Color {
    fn wow(a: u8, b: u8, c: u8) -> Color {
        Color {
            red: a,
            green: b,
            blue: c
        }
    }

    fn crazy(&self) -> (u8, u8, u8) {
        (self.red, self.green, self.blue)
    }

    fn change_red(&mut self, red: u8){
        self.red = red;
    } 

    fn stringed(self) -> String {
        format!("{}, {}, {}", self.red, self.blue, self.green)
    }   
}

// Tuple Struct

// struct Color(u8, u8, u8);


pub fn run() {
    // let color = Color {  // For traditional Struct
    //     red: 32,
    //     green: 13,
    //     blue: 89
    // };

    // let color = Color(183, 25, 35);  // For Tuple Struct

    let mut color = Color::wow(183, 25, 35); 

    // println!("Color: rgb{:?}", (color.red, color.green, color.blue));
    println!("Color: rgb{:?}", color.crazy());
    color.change_red(91);
    println!("Color: rgb{:?}", (color.crazy()));
    println!("{}", color.stringed());

    let string = "string";
    let d = 'd';
    println!("{}, {}", string, d);
    println!("{:?}", (string, d));  // you see what the debug trait does? 2 things
}