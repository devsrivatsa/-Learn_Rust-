/*
- Declarative macros are match statements on steroids.
- This is the template for creating macros.
- The lhs is input, and RHS is output.
- Each rule should end with a semicolon.

Here, nothing burger does nothing.
*/
macro_rules! nothing_burger {
    () => {};
    //() is called a capture. It is like a variable but its type is a code fragment type
}

//example
macro_rules! hello_ferris {
    () => { //can have no arguments
        println!("Hello World")
    };

    ("Ferris") => { //can have any literal as argument
        println!("Hello Ferris")
    };
    (1) => {
        println!("what 1 ??")
    };
}

macro_rules! hello_x {
    //order matters
    ($s:literal) => { println!("Hello, {}", $s) }; //literal capturs literals
    ($s:ident) => { println!("Hello, {}", $s) }; //capturs identifiers
    ($s:expr) => { println!("Hello, {}", $s) }; //expressions capture both literals and identifiers
}



fn main() {
    hello_ferris!();
    hello_ferris!("Ferris");
    hello_ferris!(1);
    hello_x!("Srivatsa");
}