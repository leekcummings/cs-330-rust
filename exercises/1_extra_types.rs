// Import HashMap from std
use std::collections::HashMap;

fn main() {
    // Hex, Octal, and Binary
    let hex = 0x04; // 0x for hex
    let octal = 0o04; // 0o for octal
    let binary =  0b0100; // 0b for binary
    // Adding different bases results in a base 10 int
    println!("Int: {}", hex + octal + binary);

    // Hashmap
    let mut person = HashMap::new(); // Create empty, mutable HashMap
    person.insert("first_name", "Lee"); // Add key value pair
    // Use `{:#?}` to pretty print the hashmap
    println!("{:#?}", person);

    // Tuple
    // Multiple variables stored in a single tuple
    let chess: (&str, char, i32) = ("Queen", 'f', 3);
    // Index tuples using dot notation
    println!("{} to {}{}", chess.0, chess.1, chess.2);

    // Array
    // Declare and quantity of the type of the items
    let primes: [i32; 4] = [2, 3, 5, 7];
    // Complex types cannot be printed using `{}` in Rust
    // println!("{}", primes);
    // Use the Debug formatter `{:?}` instead
    println!("{:?}", primes);
}