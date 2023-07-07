/*
Rc stands for reference counting - smart pointer
Usage: Rc<T> enables multiple owners for the same data. The value will only be cleaned up from menory after the last owner is done with the value
Box<t> and RefCell<T> have single owners
*/

use List::{Cons, Nil};
use std::rc::Rc; //this is the reference counting smart pointer
//Rc only allows multiple part of the program to read the same value but not modify it

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    //the Cons variant owns the data
    let list_a = Rc::new(
        Cons(5, Rc::new(
            Cons(10, Rc::new(
                Cons(15, Rc::new(Nil))))))); //now this can be cloned
    println!("initial ref count of list_a: {}", Rc::strong_count(&list_a));
    //clone does not deep copy the data. It just increments the reference count

    let list_b = Cons(30, Rc::clone(&list_a)); //connects to list_a
    let list_c = Cons(50, Rc::clone(&list_a)); //connects to list_a
    println!("ref count of list_a after creating lists b and c: {}", Rc::strong_count(&list_a));

    {
        println!("Inside the inner scope..");

        let list_d = Rc::new(Cons(100, Rc::clone(&list_a))); //connects to list_a
        println!("ref count of list_a after creating lists_d: {}", Rc::strong_count(&list_a));
        
        //clone here does not duplicate the data. It just increments the reference count
        let list_e = Rc::new(Cons(500, Rc::clone(&list_d))); //connects to list_d
        let list_f = Rc::new(Cons(5000, Rc::clone(&list_e))); //connects to list_e
        println!("ref count of list_a after creating lists e and f: {}", Rc::strong_count(&list_a));
        println!("ref count of list_d after creating lists_e: {}", Rc::strong_count(&list_d));
        println!("ref count of list_e after creating lists_f: {}", Rc::strong_count(&list_e));
    }
    println!("After leaving the inner scope, the reference count for list_a is: {}", Rc::strong_count(&list_a));

}