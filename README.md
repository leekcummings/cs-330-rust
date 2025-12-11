# CS 330 Rust Tutorial
> Welcome to Rust! 🦀

<div align="center">
    <a href="README.md"><b>History and Installation</b></a> |
    <a href="1_data_types.md">Data Types</a> |
    <a href="2_loops_conditionals.md">Loops and Conditionals</a> |
    <a href="3_functions.md">Functions</a> |
    <a href="4_classes.md">Classes (Structs)</a>
</div>

## History of Rust
Rust was created in 2006 by Mozilla software developer Graydon Hoare. It focuses on reducing memory safety-related bugs and crashes in C and C++. After being sponsored by Mozilla Research in 2009, the 1.0 version of Rust was officially released in May 2015.[^1]
Rust is a general-purpose language that can be used to create a wide range of tools and software. Because of Rust's memory safety features and fast performance, the language is often used to build operating systems, web applications, and command-line tools.[^2]

To get started programming in Rust, we'll mainly utilize the [Rust Language Website](https://www.rust-lang.org/learn), which contains documentation, links to (free!) beginner courses, and an (also free!) introductory book. In this repo, I'll mainly be referencing the [Rust Programming Language Book](https://doc.rust-lang.org/book/) and the [Standard Library Documentation](https://doc.rust-lang.org/std/index.html).

## Getting Started
### Install Rust
If you're running a macOS or Linux system, open your preferred terminal and run this command:
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Congrats! You've installed Rust. It's that easy!

If you're using a Windows computer, you can follow [these instructions](https://forge.rust-lang.org/infra/other-installation-methods.html) to install the language. It's a bit more complicated, but worth the extra effort!

### Programming Environment
Rust does not have a recommended IDE, although some IDEs may not be suited for Rust development. Using Rust requires frequent terminal access, so an IDE with easy access to terminals is crucial. You can also choose to skip the IDE entirely, opting for a command line-based text editor like Vim.

I'll be using a combination of VSCode, which has a few Rust extensions, and Nano, a CLI text editor on Debian.

## How to Run "Hello World!"
> View this section's exercise at [hello_world.rs](exercises/00_hello_world.rs)

To run a basic Hello World program, start by creating a `hello_world.rs` file to hold the program. Add this `main()` function containing a `println!` macro to the document.
```
fn main() {
    println!("Hello, world!");
}
```
Then, save the file and open a terminal window in your current directory.
Finally, run the following commands in your terminal:
```
rustc hello_world.rs # Compile the program
./hello_world # Run the compiled code
```
Congratulations, you've coded in Rust!

### How to Write Comments
Add comments to your program using either `//` or `/* */`.
Comments using `//` will only span one line, while `/* */` comments span multiple.
```
// This is a one-line comment
/* THIS is a multi-
line comment. It spans
multiple lines */
```

[^1]: https://www.technologyreview.com/2023/02/14/1067869/rust-worlds-fastest-growing-programming-language/
[^2]: https://github.blog/developer-skills/programming-languages-and-frameworks/why-rust-is-the-most-admired-language-among-developers/#what-is-rust-commonly-used-for
