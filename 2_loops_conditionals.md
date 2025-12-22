# Loops and Conditionals
> If you like Rust, use it again and again and again.

<div align="center">
    <a href="README.md">History and Installation</a> |
    <a href="1_data_types.md">Data Types</a> |
    <a href="2_loops_conditionals.md"><b>Loops and Conditionals</b></a> |
    <a href="3_functions.md">Functions</a> |
    <a href="4_classes.md">Classes (Structs)</a>
</div>

## Control Flow
Rust has multiple ways to alter the flow of programming. Using `if`, `else if`, and `else` statements as well as loops (`loop`/`for`/`while`), you can restrict the paths code will take based on selected conditions. All of these structures use curly brackets to delimit.

### Booleans
As mentioned in [Data Types](1_data_types.md), booleans are either `true` or `false` in Rust.
```
let is_rust: bool = true;
```

## If Expressions
To start an if statement, write `if` followed by the statement (like `num = 10`). Parentheses are not required for if statements in Rust. You can add additional conditions using `else if`. To catch remaining cases, use `else` after all of the previous conditions.
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
// Never reach this function
fn is_two(n: i32) -> bool {
    if n == 2 {
        println!("That's 2, alright!");
    }
    return n == 2;
}

fn main() {
    let num: i32 = 2;
    let other_condition: bool = false;

    if other_condition && is_two(num) {
        println!("Cool number!"); // Never reach this line because condition is not met
    } else {
        println!("Lame number...");
    }
}
```

## Dangling Else
If there are multiple `if` statements, followed by an `else`, Rust will link the dangling else to the most recent if statement. View the example below, where `num` doesn't meet the condition for the second `if` statement and continues into the `else`.
```
let num: i32 = 2;
if num == 2 {
    println!("You got here!");
}
if num == 5 {
    println!("You didn't get here...");
} else {
    println!("...but you did get here!");
}
``` 

## Match Statements
In Rust, you can use a `match` statement in place of a switch or case statement. Simply write match followed by the variable of interest, then write each of the different cases and their outcomes, separated by an arrow `=>`. Different conditions are separated with commas. Match statements must encompass all possible values, so you can use `_` to represent any value not included in the previous statements.
```
match num {
    1 => println!("Yay");
    2 | 3 => println("Wow");
    _ => println("Lame...");
}
```