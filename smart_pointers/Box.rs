//Box<T> allows immutable or mutable borrows checked at compile time
//Rc<T> only allows immutable borrows checked at compile time
//RefCell<T> allows immutable or mutable borrows checked at runtime

// box is useful for 2 things
    //1. store data on the heap instead of stack
    //2. with recursive types

//why should we store data on heap ?
    // this is because rust needs to know the size of something stored in stack the compile time.
    // there can be many objects that can have dynamic sizes in compile time. hence storing these in heap is more useful.
    // Hence, box is a pointer to something that is stored on heap


//usecase 1
pub trait Vehicle {
    fn drive(&self);
}

pub struct Truck;
pub struct Car;

impl Vehicle for Truck {
    fn drive(&self) {
        println!("The truck os driving")
    }
}

//usecase 2
use List::{Cons, Nil};

//this is a recursive enum
//Cons stores a value and another recursive enum
//this goes on indefinitely and there is no way to know the memory this data structure in compile time
//(cont..)
// enum List {
//     Cons(i32, List),
//     Nil
// }



fn main() {
    //usecase 1
    let t: Box<dyn Vehicle>; // now t can be a car or a truck. hence the compiler does not know the size
    t = Box::new(Truck);
    t.drive();

    //usecase 2
    //hence we use a box to wrap to list in Cons
    //instead of the following
    //let list = Cons(1, Cons(2, Cons(3, Nil)));
    //we can do the following
    let mylist = Cons(1, 
        Box::new(Cons(2, 
            Box::new(Cons(3, 
                Box::new(Cons(4,
                Nil
            )))))));
    //now even though mylist can go on forever, and store an arbitary amount of data, it is still a fixed size in stack as Box stores in stack
    //references do not own data; they do not have special capabilities.

}