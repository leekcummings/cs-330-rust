# Data Types
> Make sure to type the type of data you want to type.

<div align="center">
    <a href="README.md">History and Installation</a> |
    <a href="1_data_types.md"><b>Data Types</b></a> |
    <a href="2_loops_conditionals.md">Loops and Conditionals</a> |
    <a href="3_functions.md">Functions</a> |
    <a href="4_classes.md">Classes (Structs)</a>
</div>

Rust is a statically and explicitly typed language. Once a variable is declared, it cannot change its type. Although the compiler can infer the type of unlabeled variables, it's best to always explicitly declare variable types to avoid future confusion. Rust is also strongly typed, making mixed-type operations like combining Strings and ints impossible, without explicit conversion.

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
> Available in the bonus exercise: [bonus_types.rs](exercises/1_extra_types.rs)
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

Both are used in programming, but they differ in their functions and abilities. The [Rust Programming Language Book: Chapter 4](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html) has information about ownership and Strings.

To create a String in Rust, use `String::new()` notation. &strs are easier to create—just write anything in double quotations, and now you’ve made one.
```
let string: String = String::new();
let not_a_string: &str = "Hello ";
```

### Characters
Chars represent a Unicode character. Simply place any letter or symbol into single quotes to create a `char` variable.
```
let rust: char = '🦀';
```

### Booleans
Booleans are either `true` or `false` in Rust. That's... about it.

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
> Follow along with this section's exercise: [bonus_types.rs](exercises/1_extra_exercises/11_bonus_types.rstypes.rs)

### HashMaps
Although they're not a part of the core language, `HashMap` can be imported from `std::collections`. Creating new HashMaps is similar to creating new Strings (`HashMap::new()`). The `insert()` function is used to add key-value pairs.
```
let mut person = HashMap::new();
person.insert("first_name", "Lee");
```

### Tuples
Tuples contain a set number of items. Each item in a tuple can be a unique type.
```
let chess: (&str, char, i32) = ("Queen", 'f', 3);
```

### Arrays
Arrays can only contain one type of variable and a specific number of elements. They can be mutable, but the type and length of the array cannot change.
```
let primes: [i32; 4] = [2, 3, 5, 7];
```
For arrays without a fixed length a `Vec` can be used. Vecs are included in the std library.

## Binding Time
All variables and operations have a compile-time binding in Rust.[^1] Because Rust is statically and explicitly typed, the compiler must know the type and address of the variable before any machine code is executed.

[^1]: https://stackoverflow.com/a/49349781