//deref lets us access the value of a variable through its reference in order to compare that value
// this allows to compare without changing ownership
//smart pointers and implement drop trait and deref trait

use std::ops::Deref;


fn main() {
    let x = 5;
    let y = &x; //y is a reference to x

    assert_eq!(x, 5); //this will work
    // assert_eq!(x, y); //this will error out because we are comparing a concrete type to a reference
    //hence we can use a deref operator on y

    assert_eq!(x, *y); //this will allow the comparison
    //this is possible because the primitive type implements the deref 

    //we can also use a Box because box implements the deref trait
    let z = Box::new(x);
    assert_eq!(x, *z);

    //testing the custom implemented smart pointer
    let a = MyBox::new(x);
    assert_eq!(x, *a); 

    let m = MyBox::new(String::from("Rust"));
    hello(&m); //deref cohersion happnes here automatically as m is a smart pointer and not a string slice which is expected by hello function
    // this is because the MyBox implements deref. *m will give out a String. String in Rust also implements deref.
    // hence implicitly 2 deref's are called to get a string ref ->> &MyBox<String> -> &String -> &str
    // except for immutable to mutable cohersion, every other type cohersion is allowed. 
    // therefore, these are allowed: mut -> mut; mut -> immut; immut -> immut

}

//implementing a custom smart pointer
struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T> {
    type Target = T;
    
    fn deref(&self) -> &T {
        &self.0
    }
}

//similarly there is a mutable dref => DerefMut


//deref coersion will convert a reference of a type A to reference of type B
fn hello(name: &str) {
    println!("hello, {}!", name);
}
