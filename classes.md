# Classes in Rust
Rust doesn't have classes like Java or Python. Instead, Rust has something classes Structs.
Structs do not have methods, unlike classes in other programming languages. They also cannot inherit traits from a parent Struct.
### Creating a Struct
To make a Struct in Rust, simply write `struct` followed by the name of the class. Generally, you'll want to write the name using PascalCase. Then, write the name of each trait, followed by the type of the variable.
```
struct Person {
    name: String,
    age: i32
}
```
To get the value of a Struct's attribute, reference the name of the instance (`p1`) followed by a period and the name of the attribute (`name`).
```
let p1_name = p1.name;
```
### Using Trait and Impl to Add Methods
Although Structs don't have methods of their own, we can use Traits to add methods to a Struct. An example Trait is shown below to make a `to_string()` function.
```
trait ToString {
    fn to_string(&self) -> String;
}
```
To then give the Struct access to this function, we will use Impl, or implement the function. To implement a function a function, write `impl {trait} for {struct}` or in our example `impl ToString for Person`. You'll notice our ToString trait has a function inside. We're going to flesh out what the function does in this chunk of code, by return Person.name.
```
impl ToString for Person {
    fn to_string(&self) -> String {
        return self.name.clone()
    }
}
```
Now we can print out an instance of a Person struct by using the .to_string() method!
```
println!("{}", p1.to_string())
```