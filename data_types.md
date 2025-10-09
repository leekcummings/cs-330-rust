# Data Types
Rust is a statically typed and explicitly language. Although the compiler can infer what type an unlabeled variable is, it's best to always label your variables to avoid future confusion. I actually couldn't find any information on if Rust is strongly typed or weakly typed.<br>
Download the [data_types.rs](exercises/1_data_types.rs) exercise or create your own code based on the tutorial below. 
## Keywords
Rust has numerous strict keywords that cannot be used to name variables or attributes. Included in this list are `if`, `loop`, `self`, and `true`, which are all words used in frequently in programming. According to the [Rust Programming Language Book](https://doc.rust-lang.org/book/), there are 52 total reserved words.
## Scalar types
### Integers
To create an integer, first determine if the variable will be signed (depicted with an `i`) or unsigned (`u`). Then, choose the number of bits (`8`, `16`, `32`, `64`, or `128`). Rust also allows you to use `size` instead of a number of bits, which will either be a 32-bit  or 64-bit integer based on your computer's architecture.<br>
You can also use hex, octal, and binary integers, by using an appropriate prefix for the variable declaration (see below).
```
let x: u8 = 4; // 8 bit, non-negative integer
// If explicit typing excluded, Rust defaults to creating an i32 int 
let hex = 0x04; // Use 0x for hex
let octal = 0o04; // Use 0o for octal
let binary =  0b0100; // Use 0b for binary
```
### Floats
Floats are defined similarly to integers, using either `f32` or `f64` to create a 32 or 64-bit signed float. By default, 64 bits are used.
Integers and floats share operations of addition, subtraction, multiplication, division, and remainder.  To perform division with a remainder, the divisor must be a float.
```
let add = 1 + 2;
let sub = 3.0 - 1.0;
let mult = 2 * 3;
let div = 10 / 3; // Will return 3
let div2 = 10.0 / 3.0; // Will return 3.33...
let rem = 10 / 3; // Will return 1
```
### Characters
Chars represent a single unicode character. You can use a letter from a language's alphabet or even emojis when creating a `char` variable.
```
let heart: char = '❤️';
```
## Mutability
By default, every variable is immutable upon creation. If you want to create a variable that can be changed, add a `mut` before the variable declaration. It's recommended to avoid using `mut` unless necessary for the program.
```
let x: i32 = 5;
x = 3; // This will throw an error, as x is immutable

let y: i32 = 5;
y = 5; // Since y is mutable, no error occurs 
```
Constants also exist in Rust, creating using the `const` keyword. Constants can never be mutable and must be type annotated. They can be accessed globally.
## Mixed Type Operations
Although integers and floats are both numbers, they cannot be added together without casting, or converting one of the variable types.
```
let x = 4;
let y = 2.5;
let z = 4 + y as i32;
```
Arrays can only contain one type of variable and a specific number of elements.
## Compound Variables
Tuples contain a set number of items, but each item can be a different type.
```
let tup: (u16, f32, i32) = (500, 6.4, 1000);
```
Arrays, on the other hand, must only consist of the same variable type. They also have a fixed length.
```
let primes: [i32; 4] = [2, 3, 5, 7];
```
## Binding Time
All variables and operations have a compile time binding in Rust. Because of Rust being statically typed and explicitly typed, the compiler does the majority of the work to link variables to values.