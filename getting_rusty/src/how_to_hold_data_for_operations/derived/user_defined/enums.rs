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

fn rust_book() {
    // Create an `enum` to classify a web event. Note how both
// names and type information together specify the variant:
// `PageLoad != PageUnload` and `KeyPress(char) != Paste(String)`.
// Each is different and independent.
enum WebEvent {
    // An `enum` variant may either be `unit-like`,
    PageLoad,
    PageUnload,
    // like tuple structs,
    KeyPress(char),
    Paste(String),
    // or c-like structures.
    Click { x: i64, y: i64 },
}

// A function which takes a `WebEvent` enum as an argument and
// returns nothing.
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // Destructure `c` from inside the `enum` variant.
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        // Destructure `Click` into `x` and `y`.
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        },
    }
}

    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` creates an owned `String` from a string slice.
    let pasted  = WebEvent::Paste("my text".to_owned());
    let click   = WebEvent::Click { x: 20, y: 80 };
    let load    = WebEvent::PageLoad;
    let unload  = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
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

    rust_book();

}