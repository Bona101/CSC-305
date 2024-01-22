// mod how_to_hold_data_for_operations;
// use how_to_hold_data_for_operations::derived::*;
// // use tests::submodule::test_function;

// use crate::how_to_hold_data_for_operations::derived;

// // fn arrw(mut arr: &[i32]){
// //     &arr[0] = 9;
// // }

// // fn clo<F : FnOnce() -> Option<i32> (g: F) {
// //     println!("{}", g());
// // }

// mod cli;
// fn main() {
//     // test_one();
//     // user_defined::shapes_excercise::excercise();
//     //     let mut arrq = [1,2,3,4,5];
//     //     arrw(&arrq);
//     //     println!("{}",arrq[0]);
//     // let mut v1: Vec<i32> = vec![4];
//     // let v = v1.pop();
//     // #[derive(Debug)]

//     // user_defined::enums::run();
//     // cli::run();

//     // user_defined::unions::run();

//     //     let foo = Some(89);
//     // match foo {
//     //     Some(n) => println!("number is {}", n),
//     //     None => println!("there is no number"),
//     // }

//     // let life = 42;
//     // dbg!(life);

//     // #[derive(Debug)]
//     // struct Name<'a> {
//     //     name: &'a str,
//     // }

//     // let name = String::from("Bob");
//     // let n = Name { name: &name };

//     // fn debug_print<T: std::fmt::Debug>(t: T) {
//     //     println!("{t:?}");
//     // }

//     // #[derive(Debug)]
//     // struct Sample{a: i32};

//     // struct Whoops;

//     // debug_print(Sample{a: 1});
//     // debug_print("test");

//     // debug_print(Whoops);  // compiler error

//     // struct Gen<R> {
//     //     a: R,
//     //     b: i32
//     // }

//     // let s = Gen{
//     //     a: "qwe".to_string(),
//     //     b: 54
//     // };

//     // #[derive(Debug)]
//     // struct Container<T> {
//     //     inner: Vec<T>,
//     // }

//     // impl<T> Container<T> {
//     //     pub fn push(&mut self, item: T) {
//     //         self.inner.push(item);
//     //     }

//     //     pub fn pop(&mut self) {
//     //         self.inner.pop();
//     //     }
//     // }

//     // let mut container = Container { inner: vec![] };
//     // container.push("sample");
//     // container.push("s");
//     // container.push("sm");
//     // container.push("sa");
//     // container.pop();

//     // println!("{container:?}")

//     // let nums = vec![1, 2, 3];
//     // let doubled: Vec<i32> = nums.iter().map(|n| n * 2).collect();

//     // println!("{doubled:?}");
//     // println!("{:?}", nums);

//     // let mut a = "w".to_string();
//     // let mut b = &mut a;
//     // b.push('p');
//     // println!("{}", b);
//     // println!("{}", a);
//     // derived::functions::everything_in_closures::run4();

//     // let list = vec![1, 2, 3];
//     // println!("Before defining closure: {:?}", list);

//     // fn m(list: Vec<i32>) -> Vec<i32> {
//     //     list
//     // }

//     // m(list);
//     // let only_borrows = || println!("From closure: {:?}", list);

//     // println!("Before calling closure: {:?}", list);
//     // only_borrows();

//     // println!("After calling closure: {:?}", list);
//     // println!("After calling closure: {:?}", list);

//     // #[derive(Debug)]
//     //     struct MyType {
//     //         value: i32,
//     //         vale: i32,
//     //     }

//     // impl From<i32> for MyType {
//     //     fn from(value: i32) -> Self {
//     //         MyType { value, vale: 7 }
//     //     }
//     // }

//     // let my_instance: MyType = 42.into();
//     // println!("{my_instance:?}")

//     // let a: char = '1';
//     // let ab = a as u8;
//     // println!("{ab}");

//     // let a = "q";
//     // let w = String::from(a);

//     // println!("{a} {w}");

//     // fn divide(x: f64, y: f64) -> Result<f64, String> {
//     //     if y == 0.0 {
//     //         Err("Division by zero".to_string())
//     //     } else {
//     //         Ok(x / y)
//     //     }
//     // }

//     // match divide(10.0, 32.0) {
//     //     Ok(result) => println!("Result: {}", result),
//     //     Err(err) => println!("Error: {}", err),
//     // }

//     /////////////////////////////////////////////////////////////////////////

//     // fn read_file_ok() -> Result<String, std::io::Error> {
//     //     let content = std::fs::read_to_string(
//     //         "C:\\Users\\bonav\\OneDrive\\Documents\\Codes\\CSC 305\\CSC-305\\src\\pius.rs",
//     //     )?;
//     //     Ok(content) // returns Ok("whatever is in pius.rs. Must be in String")
//     // }

//     // fn read_file_err() -> Result<String, std::io::Error> {
//     //     let content = std::fs::read_to_string(
//     //         "C:\\Users\\bonav\\OneDrive\\Documents\\Codes\\CSC 305\\CSC-305\\src\\pis.rs",
//     //     )?; // No such file so returns early as an Err(with the message that must not be in String even though the type of variable taking the value is of type String)
//     //     Ok(content)
//     // }

//     // fn read_file_2() -> Result<String, String> {
//     //     Err(Err(64.to_string())?) // returns Err("64")
//     // }

//     // fn read_file_3() -> Result<String, String> {
//     //     Err(Ok::<String, String>("s".to_string())?) // returns Err("s")
//     // }

//     // fn read_file_4() -> Result<String, String> {
//     //     // let content: String = std::fs::read_to_string("C:\\Users\\bonav\\OneDrive\\Documents\\Codes\\CSC 305\\CSC-305\\src\\pius")?;
//     //     Ok(Ok::<String, String>("s".to_string())?) // returns Ok("s")
//     // }

//     // fn read_file_5() -> Result<String, String> {
//     //     // let content: String = std::fs::read_to_string("C:\\Users\\bonav\\OneDrive\\Documents\\Codes\\CSC 305\\CSC-305\\src\\pius")?;
//     //     Ok(Err("s")?) // returns Err("s")
//     // }

//     // println!("{:?}", read_file_ok());
//     // println!("{:?}", read_file_err());
//     // println!("{:?}", read_file_2());
//     // println!("{:?}", read_file_3());
//     // println!("{:?}", read_file_4());
//     // println!("{:?}", read_file_5());

//     /////////////////////////////////////////////////////

//     // let some_result = [4];

//     // let result = some_result.iter().map(|x| x * 2).and_then(|x| divide_by_two(x));

//     // #![allow(dead_code)]

//     // #[derive(Debug)]
//     // enum Food {
//     //     CordonBleu,
//     //     Steak,
//     //     Sushi,
//     // }
//     // #[derive(Debug)]
//     // enum Day {
//     //     Monday,
//     //     Tuesday,
//     //     Wednesday,
//     // }

//     // // We don't have the ingredients to make Sushi.
//     // fn have_ingredients(food: Food) -> Option<Food> {
//     //     match food {
//     //         Food::Sushi => None,
//     //         _ => Some(food),
//     //     }
//     // }

//     // // We have the recipe for everything except Cordon Bleu.
//     // fn have_recipe(food: Food) -> Option<Food> {
//     //     match food {
//     //         Food::CordonBleu => None,
//     //         _ => Some(food),
//     //     }
//     // }

//     // // To make a dish, we need both the recipe and the ingredients.
//     // // We can represent the logic with a chain of `match`es:
//     // fn cookable_v1(food: Food) -> Option<Food> {
//     //     match have_recipe(food) {
//     //         None => None,
//     //         Some(food) => have_ingredients(food),
//     //     }
//     // }

//     // // This can conveniently be rewritten more compactly with `and_then()`:
//     // fn cookable_v3(food: Food) -> Option<Food> {
//     //     have_recipe(food).and_then(have_ingredients)
//     // }

//     // // Otherwise we'd need to `flatten()` an `Option<Option<Food>>`
//     // // to get an `Option<Food>`:
//     // fn cookable_v2(food: Food) -> Option<Food> {
//     //     have_recipe(food).map(have_ingredients).flatten()
//     // }

//     // fn eat(food: Food, day: Day) {
//     //     match cookable_v2(food) {
//     //         Some(food) => println!("Yay! On {:?} we get to eat {:?}.", day, food),
//     //         None => println!("Oh no. We don't get to eat on {:?}?", day),
//     //     }
//     // }

//     // // fn main() {
//     // let (cordon_bleu, steak, sushi) = (Food::CordonBleu, Food::Steak, Food::Sushi);

//     // eat(cordon_bleu, Day::Monday);
//     // eat(steak, Day::Tuesday);
//     // eat(sushi, Day::Wednesday);

//     // fn x() -> Option<i32> {
//     //     Some(None::<i32>?)
//     // }

//     // println!("{:?}", x());
//     // }
//     // mod tests;

//     // #[cfg(test)]
//     // mod tests {
//     //     mod submodule {
//     //         #[test]
//     //         fn test_function() {
//     //             // Test logic
//     //             for i in 0..1_000_000 {
//     //                 panic!("qq"); // run with cargo test but that doesn't run the functions it justs "tests" them so even though this panic is here, the program doesn't really terminate with an error (I think)
//     //                 println!("wow")
//     //             }
//     //         }
//     //     }
//     // }
//     // // tests::submodule::test_function();

//     // fn print_value<T>(value: T) -> T {
//     //     value
//     // }

//     // println!("{}", print_value(42));
//     // println!("{}", print_value("Hello, Rust!"));

//     //////////////////////////////////////

//     // #[derive(Debug)]
//     // struct Pair<T, U> {
//     //     first: T,
//     //     second: U,
//     // }

//     // let pair_int_str = Pair {
//     //     first: 42,
//     //     second: "rust",
//     // };
//     // let pair_float_char = Pair {
//     //     first: 3.14,
//     //     second: 'a',
//     // };

//     // println!("{:?}", pair_int_str.second);
//     // println!("{:?}", pair_float_char.second);

//     ///////////////////////////////////////////////////////
//     // for i in 1..=5 {
//     //     // Code to repeat 5 times
//     //     println!("{i}");
//     // }
//     //////////////////////////////////////////////////////////////

//     // trait MyTrait {
//     //     const MY_CONSTANT: i32;

//     //     fn print_constant(&self) {
//     //         println!("Constant: {}", Self::MY_CONSTANT);
//     //     }
//     // }

//     // struct MyStruct;

//     // impl MyTrait for MyStruct {
//     //     const MY_CONSTANT: i32 = 42;
//     // }

//     // let my_instance = MyStruct;
//     // my_instance.print_constant();
//     ////////////////////////////////////////////////////////////////////

//     // // Integer addition
//     // println!("1 + 2 = {}", 1u32 + 2);

//     // // Integer subtraction
//     // println!("1 - 2 = {}", 1i32 - 2);
//     // // TODO ^ Try changing `1i32` to `1u32` to see why the type is important

//     // // Scientific notation
//     // println!("1e4 is {}, -2.5e-3 is {}", 1e4, -2.5e-3);

//     // // Short-circuiting boolean logic
//     // println!("true AND false is {}", true && false);
//     // println!("true OR false is {}", true || false);
//     // println!("NOT true is {}", !true);

//     // // Bitwise operations
//     // println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
//     // println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
//     // println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
//     // println!("1 << 5 is {}", 1u32 << 5);
//     // println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

//     // // Use underscores to improve readability!
//     // println!("One million is written as {}", 1_000_000u32);

//     //////////////////////////////////////////////////////////////////////////

//     // Suppress all warnings from casts which overflow.
//     #![allow(overflowing_literals)]

//     let decimal = 65.4321_f32;

//     // Error! No implicit conversion
//     // let integer: u8 = decimal;
//     // FIXME ^ Comment out this line

//     // Explicit conversion
//     let integer = decimal as u8;
//     let character = integer as char;

//     // Error! There are limitations in conversion rules.
//     // A float cannot be directly converted to a char.
//     // let character = decimal as char;
//     // FIXME ^ Comment out this line

//     println!("Casting: {} -> {} -> {}", decimal, integer, character);

//     // when casting any value to an unsigned type, T,
//     // T::MAX + 1 is added or subtracted until the value
//     // fits into the new type

//     // 1000 already fits in a u16
//     println!("1000 as a u16 is: {}", 1000 as u16);

//     // 1000 - 256 - 256 - 256 = 232
//     // Under the hood, the first 8 least significant bits (LSB) are kept,
//     // while the rest towards the most significant bit (MSB) get truncated.
//     println!("1000 as a u8 is : {}", 1000 as u8);
//     // -1 + 256 = 255
//     println!("  -1 as a u8 is : {}", (-1000i32) as u8);

//     // For positive numbers, this is the same as the modulus
//     println!("1000 mod 256 is : {}", -1000 % -256);

//     // When casting to a signed type, the (bitwise) result is the same as
//     // first casting to the corresponding unsigned type. If the most significant
//     // bit of that value is 1, then the value is negative.

//     // Unless it already fits, of course.
//     println!(" 128 as a i16 is: {}", 128 as i16);

//     // In boundary case 128 value in 8-bit two's complement representation is -128
//     println!(" 128 as a i8 is : {}", 129 as u8);

//     // repeating the example above
//     // 1000 as u8 -> 232
//     println!("1000 as a u8 is : {}", 1000 as u8);
//     // and the value of 232 in 8-bit two's complement representation is -24
//     println!(" 232 as a i8 is : {}", 1112 as i8);

//     // Since Rust 1.45, the `as` keyword performs a *saturating cast*
//     // when casting from float to int. If the floating point value exceeds
//     // the upper bound or is less than the lower bound, the returned value
//     // will be equal to the bound crossed.

//     // 300.0 as u8 is 255
//     println!(" 300.0 as u8 is : {}", 300.0_f32 as u8);
//     // -100.0 as u8 is 0
//     println!("-100.0 as u8 is : {}", -100.0_f32 as u8);
//     // nan as u8 is 0
//     println!("   nan as u8 is : {}", f32::NAN as u8);

//     // This behavior incurs a small runtime cost and can be avoided
//     // with unsafe methods, however the results might overflow and
//     // return **unsound values**. Use these methods wisely:
//     unsafe {
//         // 300.0 as u8 is 44
//         println!(" 300.0 as u8 is : {}", 300.0_f32.to_int_unchecked::<u8>());
//         // -100.0 as u8 is 156
//         println!(
//             "-100.0 as u8 is : {}",
//             (-100.0_f32).to_int_unchecked::<u8>()
//         );
//         // nan as u8 is 0
//         println!("   nan as u8 is : {}", f32::NAN.to_int_unchecked::<u8>());
//     }
    

    
// }

fn main(){
    print!("y");
    print!("y");
}