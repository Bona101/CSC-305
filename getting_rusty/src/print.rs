pub fn run() {
    // Print to console
    println!("Hello from the print.rs file");

    // Basic Formatting
    println!("{}", 1);
    println!("{} likes to code in {}", "He", "school" );

    // Positional arguments
    println!("{0} likes to code in {1}", "he", "school" );
    
    // Basic formatting and positional arguments, empty braces indicates {0}
    println!("{1} likes to code {} in {2}", "python", "he", "school" );

    // Named arguments
    println!(
        "{boy} likes to code in {school}", 
        boy = "Emeka", 
        school = "PAU" );

    // Placeholder traits
    println!("Binary: {:b}, Hex: {:x}, Oct: {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (15, false, "string"));

    // Basic math
    println!("10 + 5 = {}", 10 + 5);

}
