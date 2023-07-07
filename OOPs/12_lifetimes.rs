fn main() {
    let string1 = String::from("abcde");
    let string2 = String::from("xyz");

    /*
    This function will cause an error: missing lifetime specifier

    fn longest(x:&str, y:&str) -> &str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    this is because x and y can have different lifetimes and the function can return anything
    also if this function is called from many different places in the program, then the argument values can have different lifetimes
    */

    /*
    Hence we use a generic lifetime annotator 'a; it can be 'apple; or 'anything, but the convention is a tick with a lowercase letter.

    by annotating with the same 'a for x,y and return value, we do not change the lifetime of the variables. Rather, we are establishing a 
    relationship between lifetimes of x,y and return. 
    The shortest lifetime of x, y will be the lifetime of the return value.
        // &i32 --> just a ref
        // &'a i32 --> ref with an explicit lifetime
        // &'a mut i32 --> mutable ref with an explicit lifetime
    */

    fn longest<'a>(x:&'a str, y:&'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is {}", result);

    //static lifetimes live as long as the main function.
    //all string references have static lifetimes
    let s: &'static str = "I have a static lifetime";
}



//lifetimes in struct: Necessary when we assign a reference as one of its variants

struct ImportantExcerpt<'a> {
    part: &'a str,
}

// this means, this struct cannot outlive the lifetime of the part variable

/*
The lifetimes
sometimes the compiler can determine the lifetimes based of the following rules: 
i/p arguments -> input lifetimes
o/p value -> output lifetime

1. each parameter that is a reference gets its own lifetime parameter
2. if there is exactly one i/p lifetime parameter, that lifetime is assigned to all o/p parameters;
3. If there are multiple i/p lifetime parameters, but one of them is &self or &mut self, the lifetime of self is assigned to all
   o/p parameters.
*/

// we do not need to specify lifetimes to announcement and return value because of the 3rd rule
impl<'a> ImportantExcerpt<'a> {
    fn return_part(&self, announcement: &str) {
        println!("attention please: {}", announcement);
    }
}

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x:&'a str, 
    y:&'a str, 
    ann:T
) -> &'a str where T: Display {
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
} 


