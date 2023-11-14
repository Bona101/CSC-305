enum Enum {
    Foo,
    Bar,
    Baz,
}

enum Fieldless {
    Tuple(),
    Struct{},
    Unit,
}

#[repr(u8)]
enum FieldlessWithDiscrimants {
    First = 10,
    Tuple(),
    Second = 20,
    Struct{},
    Unit,
}

enum Measurement {
    Temperature(f32),
    Distance(f64),
    Weight(f64),
}

pub fn run() {
    println!("{}", Enum::Foo as u8);
    println!("{}", Enum::Bar as i64);
    println!("{}", Enum::Baz as isize);

    println!("{}", Fieldless::Tuple() as isize);
    println!("{}", Fieldless::Struct{} as isize);
    println!("{}", Fieldless::Unit as isize);

    println!("{}", FieldlessWithDiscrimants::First as u8);
    println!("{}", FieldlessWithDiscrimants::Tuple() as u8);
    println!("{}", FieldlessWithDiscrimants::Second as u8);
    println!("{}", FieldlessWithDiscrimants::Struct{} as u8);
    println!("{}", FieldlessWithDiscrimants::Unit as u8);


    let temp = Measurement::Temperature(25.5);
    let distance = Measurement::Distance(1000.0);
    let weight = Measurement::Weight(75.2);

    // Perform operations based on the type of measurement
    match temp {
        Measurement::Temperature(value) => println!("Temperature: {} degrees Celsius", value),
        Measurement::Distance(value) => println!("Distance: {} meters", value),
        Measurement::Weight(value) => println!("Weight: {} kilograms", value),
    }

    

}