## Booleans in Rust
Bools are either `true` or `false` in Rust.
```
let is_rust: bool = true;
```

## Control Flow
Rust has multiple ways to alter the flow of programming. Using if/else if/else statements and loops (loop/for/while), you can restrict if code can run based on a condition or run a section of code multiple times. All of these structures use curly brackets to delimit.
### If Expressions
To start an if statement, simply write `if` followed by the statement (ex: `num = 10`). Paranthesis are not required for if statements. Add additional conditions using `else if`. To catch remaining cases, use `else` after all previous conditions.
```
let num = 2;
if num == 10 {
    println!("Wow, that's 10!");
} else if num == 3 {
    println!("Cool number!");
} else {
    println!("Lame number...");
}
```
### Short-Circuit Evaluation
Rust uses `&&` and `||` for short-circuiting logical AND and OR.
```
fn is_two(n: i32) -> bool { // Never reach this function
    if n == 2 {
        println!("That's 2, alright!");
    }
    return n == 2;
}

fn is_true(b: bool) -> bool {
    if b {
        println!("True");
    } else {
        println!("False");
    }
    return b;
}

let num: i32 = 2;
let other_condition: bool = false;

if is_true(other_condition) && is_two(num) {
    println!("Cool number!"); // Never reach this line because condition is not met
} else {
    println!("Lame number...");
// Result
// False
// Lame number...
}
```
## Dangling Else
If there are multiple `if` statements, followed by an `else`, Rust will link the that dangling else to the most recent if statement. View the example below, where `num` doesn't meet the condition for the second if statement and continues into the `else`.
```
let num: i32 = 2;
if num == 2 {
    println!("You got here!");
} if num == 5 {
    println!("You didn't get here...");
} else {
    println!("...but you did get here!");
}
``` 
## Match Statments
In Rust, you can use a `match` statement in place of a switch or case statement. Simply write match followed by the variable of interest (ex: `match num`), then write each of the different cases and their outcomes, seperated by an arrow `=>`. Different conditions are seperated with commas. Match statements must encompass all possible values, so you can use `_` to represent any value not included in the previous statements.
```
match num {
    1 => println!("Yay");
    2 | 3 => println("Wow");
    _ => println("Lame...");
    }
```