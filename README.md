# cs-330-rust
> Welcome to Rust 🦀 This repo was created for CS 330: Programming Languages
## History
Rust was created in 2006 by Mozilla software developer, Graydon Hoare, with the focus of reducing memory-related bugs and crashes caused by languages like C and C++. After being sponsored by Mozilla Research in 2009, the 1.0 version of Rust was officially released in May 2015.[^1]
Rust is a general purpose language, which can be used to create a variety of software. Because of Rust's memory safety features and fast performance, the language is used to build operating systems, web applications, and command line tools.[^2]<br><br>
To get started programming in use, use the [Rust Language Website](https://www.rust-lang.org/learn) which contains documentation, links to free beginner courses, and an introductory book. In this repo, I'll mainly be referencing the [Rust Programming Language Book](https://doc.rust-lang.org/book/) and the [Standard Library Documentation](https://doc.rust-lang.org/std/index.html).
## Getting Started
### Install Rust
if you're running a MacOS or Linux system, open a terminal and run this command:
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Congrats! You've installed Rust.
If you're using a Windows computer, follow [these instructions](https://forge.rust-lang.org/infra/other-installation-methods.html) to install the language.
### Programming Environment
Rust does have have a recommended IDE, although, some may not be suited for Rust development. Using Rust requires frequent terminal access, so an IDE with access to multiple terminals is crucial. Some choose to skip the IDE entirely, opting for a command line-based text editor like Vim.<br><br>
If you want the features of a full IDE, use whatever works best for you. I'll be using a combination of VSCode, which has a few Rust extensions, and Nano, a CLI text editor on Linux. 
### How to Run "Hello World!"
To run a basic Hello World program, start by creating a `main.rs` file to hold the program. Type the following code:
```
fn main() {
    println!("Hello, world!");
}
```
Save the file and open a terminal in your current directory.
Then, run the following commands:
```
rustc main.rs # Compile the program
./main # Run the compiled code
```
### How to Write Comments
Add comments to your program using either `//` or `/* */`
```
// This is a one line comment
/* THIS
is a multi-
line comment */
```

[^1]: https://www.technologyreview.com/2023/02/14/1067869/rust-worlds-fastest-growing-programming-language/

[^2]: https://github.blog/developer-skills/programming-languages-and-frameworks/why-rust-is-the-most-admired-language-among-developers/#what-is-rust-commonly-used-for
