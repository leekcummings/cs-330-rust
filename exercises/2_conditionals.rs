fn is_two(n: i32) -> bool {
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


fn main() {
    let mut num: i32 = 2;
    let other_condition: bool = false;

    if num == 10 {
    println!("Wow, that's 10!");
    } else if is_two(num) && is_true(other_condition) {
        println!("Cool number!");
    } if !other_condition {
        // Do nothing
    } else {
        println!("Lame number...");
    }

    match num {
        1 => println!("Yay"),
        2 | 3 => println!("Wow"),
        _ => println!("Lame...")
    }
}