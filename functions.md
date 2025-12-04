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
In Rust, functions can be declared anywhere within a file, and this will not affect a functions ability to be called.
### Recursion
Rust supports recursion, though it is suggested to avoid using it when possible. Because of Rust's focus on memory usage, using recursion risks a stack overflow that compromises memory safety. Recursive functions can be difficult to implement in Rust because the size of a value may be unknown at compile time.
### Accepting and Returning Multiple Values
Rust functions can accept and return any number and type of variables. If you want to return multiple values, you can store them in a tuple. Tuples in Rust can have multiple variables types.
```
// An example tuple with multiple variable types
fn returnMultiple(x: i32, y: bool, z: char) {
  return (x: i32, y: bool, z: char)
}
```
Functions in Rust are pass-by value. This doesn't mean that you can't pass references into function, but you have to explictly pass in the reference value for the function to access it.
### Stack and Heap
Basic local variables including ints, floats, and booleans are stored on the stack once they function they're created in runs. When the function exits, the memory is deallocated.
For variables that exist outside of a single function's scope or variables that are passed into other functions, they are placed on the heap and referenced in the code.
Variables are not accessible outside of the scope of the function or loop they were created in. This means that if you create a variable in a function, you need to return it if you want continued access to it.