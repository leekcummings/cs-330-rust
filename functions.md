# Functions!!!!!
Creating functions in Rust is easy! Simply, write `fn`, before a function name (make sure to include parans after the name) and finish with curly brackets. Functions can accept any number of parameters with multiple different types.
```
// function definition with no params
fn function_name() {
    chess_location('A', 4); // calling another function
}
// function definition with 2 different param types
fn chess_location(letter: char, number: char) {
  ...
}
```
In Rust, functions can be declaredd anywhere within a file, and this will not affect a functions ability to be called.
### Recursion
Rust supports recursion, though it is suggested to avoid using it when possible. Because of Rust's focus on memory usage, using recursion risks a stack overflow that compromises memory safety.
### Returning Multiple Values
If you want to return multiple values, you can store them in a tuple. Tuples in Rust can have multiple variables types.
```
// An example tuple with multiple variable types
return (x: i32, y: bool, z: char)
```
