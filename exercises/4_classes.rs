struct Person {
    name: String,
    age: i32
}

// a trait to add to the Person struct, giving it a to_string() function
trait ToString {
    fn to_string(&self) -> String;
}

// structs do not have methods of their own
// Using imp, we can add the to_string() method from the trait ToString
impl ToString for Person {
    fn to_string(&self) -> String {
        return self.name.clone()
    }
}

fn main() {
    let p1 = Person{name: "Bella".to_string(), age: 21};
    // Calling the to_string() function for Person
    println!("{}", p1.to_string())
}