// Import HashMap from std
use std::collections::HashMap;

fn main() {
    // Integers
    let x: u32 = 10;
    let y = 11; // Default int type is i32

    // Strings
    let str_a: String = String::new(); // Creates an empty String
    let str_b: String = String::from("Hello World"); // Creates non-empty string

    // Float
    let a: f32 = 3.14159;
    let b = 2.7182; // Default float type is f64

    // Bool
    let is_boolcontacts: bool = true;
    let is_python: bool = false;

    // Array
    let primes: [i32; 4] = [2, 3, 5, 7]

    // Hashmap
    let mut map = HashMap::new(); // Create empty HashMap
    map.insert("first_name", "last_name") // Add key: value pair (firstname: last_name)
}