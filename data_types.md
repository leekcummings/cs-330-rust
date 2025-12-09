# Data Types
> Make sure to type the type of data you want to type. Type.

Rust is a statically and explicitly typed language. Once a variable is declared, it cannot change its type. Although the compiler can infer the type of unlabeled variables, it's best to always explicitly declare variable types to avoid future confusion. Rust is also strongly typed, making mixed type operations like combining Strings and ints impossible, without explicit conversion.

## Keywords
Rust has numerous strict keywords that cannot be used to name variables or attributes. Included in this list are `if`, `loop`, `self`, and `true`. According to the [Rust Programming Language Book](https://doc.rust-lang.org/book/), there are 52 total reserved words.

## Variable Types
> Follow along with this section's exercise: [data_types.rs](exercises/1_data_types.rs)

### Integers
To create an integer, first determine if the variable will be signed (depicted with an `i`) or unsigned (`u`). Then, choose the number of bits (`8`, `16`, `32`, `64`, or `128`). By default, `i32` ints are used. You can also use `size` instead of a number of bits, which will result in a 32-bit or 64-bit integer based on your computer's architecture.
```
let x: u16 = 10;
let y = 11; // Default int type is i32
```
> Available in the bonus exercise: [bonus_types.rs](exercises/11_bonus_types.rs)
> To make hex, octal, and binary integers, use the appropriate prefix when declaring the variable (see below).
> ```
> let base_10: u8 = 4;
> let hex = 0x04;
> let octal = 0o04;
> let binary =  0b0100;
> ```

### Floats
Floats are defined similarly to integers, using either `f32` or `f64` to create a 32 or 64-bit signed float. By default, 64 bits are used.
Integers and floats cannot be added, subtracted, multiplied, or divided together. Using type casting (`as type`), these numbers can be combined. To perform division with a remainder, both numbers must be floats.
```
let f: f32 = 3.14159;
let i: u16 = 10;
let combined = a + x as f32; // Cast u16 as f32
```

### String vs. &str
Rust distinguishes between owned strings and string literals using String and &str. A String can grow, shrink, and mutate. String literals can never be mutable. They represent a pointer to a memory location where a string begins.

Both are used in programming, but they differ in their functions and abilities. For more information, the [Rust Programming Language Book Chapter 4](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html) has information about ownership and Strings.
```
let mut string: String = String::new();
let not_a_string: &str = "Hello ";
```

### Characters
Chars represent a Unicode character. Simply place any letter or symbol into single quotes to create a `char` variable.
```
let rust: char = '🦀';
```

### Booleans
Bools are either `true` or `false` in Rust. That's... about it.

## Mutability and Constants
By default, every variable is immutable upon creation. To create a variable that can be changed, add a `mut` before the variable declaration. It's recommended to avoid using mutability unless necessary for the program (and the compiler will yell at you if you misuse it).
```
let immutable: i32 = 5;
immutable = 3; // This will throw an error

let mut string: String = String::new();
string += "This works!";
```
Constants also exist in Rust, created using the `const` keyword. Constants can never be mutable and must be type-annotated. They can be accessed globally.


## Additional Types
### HashMaps
Although they're not a part of the core language, `HashMap` can be imported from `std::collections`. Creating new HashMaps is the same as creating new Strings, and the `insert()` function is used to add key value pairs.
```
let mut person = HashMap::new();
person.insert("first_name", "Lee");
```

## Compound Variables (Tuples and Arrays)
Tuples contain a set number of items, but each item can be a different type.
```
let tup: (u16, f32, i32) = (500, 6.4, 1000);
```
Arrays can only contain one type of variable and a specific number of elements. They can become mutable, but the type and length of the array cannot change
```
let primes: [i32; 4] = [2, 3, 5, 7];
```

## Binding Time
All variables and operations have a compile-time binding in Rust. Because of Rust being statically typed and explicitly typed, the compiler does the majority of the work to link variables to values.