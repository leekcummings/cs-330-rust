fn say_hello() {
    println!("Hello!");
}

// Takes in two i32 ints, returns an i32 int
fn multiply(x: i32,y: i32) -> i32 { 
    return x * y
}

// Takes in a string literal, returns an array of 2 Strings
fn split_string(s: &str) -> (String, String, usize) {
    let mid: usize = s.chars().count() / 2; // Calculate midpoint of &str
    let start: &str = &s[..mid]; // Slice from start to mid
    let end: &str = &s[mid..]; // Slice from mid to end
    // Convert results to_string()
    // &str cannot be returned as the String was created within the function
    // `return` doesn't need to be specified as long as there's no semicolon
    (start.to_string(), end.to_string(), mid)
}

// Pass-by-value example
fn add_ten(v: i32) -> i32 {
    // Add 10 to the passed in value
    v + 10
}

fn main() {
    say_hello();
    let result: i32 = multiply(5, 6); // Save result into new variable
    println!("{}", result);
    println!("{:?}", split_string("Strings")); // Or print directly
    // Pass-by-value example
    let num: i32 = 3;
    println!("Updated: {}", add_ten(num)); // Print updated value
    println!("Original: {}", num); // Original value remains unchanged
}
