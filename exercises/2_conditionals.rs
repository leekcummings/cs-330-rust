// Never reach this function!
fn is_two(n: i32) -> bool {
    if n == 2 {
        println!("That's 2, alright!");
    }
    return n == 2;
}

fn main() {
    let num: i32 = 2;
    let other_condition: bool = false;

    // Short-circuit
    if other_condition && is_two(num) {
        println!("Cool number!"); // Never reach this line because condition is not met
    } else {
        println!("Lame number");
    }
    // Dangling else
    if other_condition {
        println!("You didn't get here...");
    } else {
        println!("...but you did get here!");
    }

    // Match statements
    match num {
        1 => println!("Yay"),
        2 | 3 => println!("Wow"),
        _ => println!("Lame...")
    }
}