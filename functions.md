# Functions
> Use them, unless you want main() to be 300 lines long.

<div align="center">
    <a href="README.md">History and Installation</a> |
    <a href="data_types.md">Data Types</a> |
    <a href="loops_conditionals.md">Loops and Conditionals</a> |
    <a href="functions.md"><b>Functions</b></a> |
    <a href="classes.md">Classes (Structs)</a>
</div>

## Declaring Functions
To create a new function in Rust, simply write `fn`, followed by the name of the function. Add open and closed parentheses and finish the declaration with curly brackets. Functions can accept any number of parameters with multiple different types. Each parameter should be listed in order with its variable type.
```
// No params
fn say_hello() {
    println!("Hello!");
}
// Multiple params
fn multiply(x: i32,y: i32) -> i32 { 
    return x * y
}
```
In Rust, functions can be declared anywhere within a file.

### To `return` or Not To `return`
In Rust, it's not required to write `return` when returning parameters. If the final line of your function doesn't have a semicolon, the compiler automatically returns that value. This can simplify code writing, but may reduce readability to those less experienced with Rust. Ultimately, it's your choice!
```
fn add_ten(v: i32) -> i32 {
    v + 10 // Returns the last line automatically
}
```

### Accepting and Returning Multiple Values
Rust functions can accept and return any number and type of variables. To return multiple values, return a tuple containing each variable. Tuples in Rust can have multiple variable types.
```
fn split_string(s: &str) -> (String, String, usize) {
    let mid: usize = s.chars().count() / 2;
    let start: &str = &s[..mid];
    let end: &str = &s[mid..];
    (start.to_string(), end.to_string(), mid)
}
```
Functions in Rust are pass-by-value. This doesn't mean that you can't pass references into functions, but you have to pass in the reference value for the function to access it.

## Using Recursion
Although Rust supports recursion, it is *highly* suggested to avoid using it. Because of Rust's focus on memory usage, utilizing recursion risks a stack overflow, compromising memory safety. Recursive functions can be difficult to implement in Rust because the size of a value may be unknown at compile time.

## Stack and Heap Allocation
Local variables and basic variable types, including ints, floats, and booleans, are stored on the stack. When the function in which a variable was created exits, the memory is deallocated.
For variables that exist outside of a single function's scope (global) or variables that are passed into other functions, they are placed on the heap.
Variables are not accessible outside of the scope of the function or loop they were created in. This means that if you create or modify a variable within a function, you must return it if you want continued access to it.