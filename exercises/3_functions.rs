fn multiply(x: i32, y: i32) -> i32 {
    return x * y
}

fn split_string(s: &str) -> [String; 2] {
    let len = s.chars().count() / 2;
    return [s[..len].to_string(), s[len..].to_string()]
}

fn main() {
    let result = multiply(5, 6);
    println!("{}", result);
    println!("{:?}", split_string("Strings"))
}