// Creating a new Person struct
struct Person {
    name: String,
    age: i32
}

// A trait to add to the Person struct
trait ToString {
    fn to_string(&self) -> String;
}

// Structs do not have methods of their own
// Using impl, we can add the to_string() method from the trait ToString
impl ToString for Person {
    fn to_string(&self) -> String {
        return self.name.clone()
    }
}

fn main() {
    // Create an instance of Person
    let p1 = Person{name: "Bella".to_string(), age: 21};
    let age = &p1.age;
    println!("Your age is: {}", age);
    // Calling the to_string() method for Person
    println!("{}", p1.to_string())
}