fn main() {
    // Integers
    // Format as `i` or `u` followed by any in [8, 16, 32, 64, 128, size]
    let x: u16 = 10;
    let y = 11; // Default int type is i32
    println!("{}", x + y); // Signed and unsigned ints can be added

    // Float
    let a: f32 = 3.14159;
    let b = 2.71828; // Default float type is f64
    println!("Float: {}", a + b);

    // Combining a float and an int will return an error.
    // To combine, convert either to the opposite's type.
    // println!("{}", a + x);

    // I'll convert the int to an f32
    // Keep in mind that floats of different sizes CANNOT be added
    println!("Int as Float: {}", a + x as f32);

    // Strings
    let string: String = String::new(); // Creates a String
    let not_a_string: &str = "Hello "; // "Hello " is a &str

    // Strings cannot be combined with Strings
    // A String can only join a &str
    println!("{}", string + not_a_string);

    // Characters
    let rust: char = '🦀'; // Remember the single quotes!
    println!("{}", rust);

    // Bool
    let is_bool: bool = true;
    let is_python: bool = false;
    println!("{}", is_bool == is_python); // Prints `false`
}