mod how_to_hold_data_for_operations;
use how_to_hold_data_for_operations::derived::*;
use how_to_hold_data_for_operations::primitives::*;




fn main() {
    println!("PRIMITIVES: SCALARS");
    scalar::scalars::run();
    println!("\nPRIMITIVES: COMPOUNDS: ARRAYS_AND_SLICES");
    compound::arrays_and_slices::run();
    println!("\nPRIMITIVES: COMPOUNDS: TUPLES");
    compound::tuples::run();
    println!("\nDERIVED: USER_DEFINED: STRUCTS");
    user_defined::structs::run();
    println!("\nDERIVED: USER_DEFINED: ENUMS");
    user_defined::enums::run();
    println!("\nDERIVED: USER_DEFINED: UNIONS");
    user_defined::unions::run();
    println!("\nDERIVED: FUNCTIONS: CLOSURES");
    functions::closures::run();

}