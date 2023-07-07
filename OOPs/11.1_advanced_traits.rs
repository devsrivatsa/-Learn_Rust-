//1. associated types vs generics
// the question is when does it make sense to have an associated type vs generic implementation ?

//here there can only be 1 concrete implementation of Iterator for Counter1
trait Iterator {
    type item;
    fn next(&mut self) -> Option<Self::item>;
}
struct Counter1 {}
impl Iterator for Counter1 {
    type item = usize;
    fn next(&mut self) -> Option<Self::item> {
        Some(0)
    }
}
//------------------------------VS------------------------------//
trait Iterator2<T> {
    fn next(&mut self) -> Option<T>;
}
struct Counter2 {}
impl Iterator2<i32> for Counter2 {
    fn next(&mut self) -> Option<i32> {
        Some(0)
    }
}
impl Iterator2<char> for Counter2 {
    fn next(&mut self) -> Option<char> {
        Some('A')
    }
}
impl Iterator2<f64> for Counter2 {
    fn next(&mut self) -> Option<f64> {
        Some(0.0)
    }
}
//----------------------------Operator Overloading--------------------------------//
use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32
}
impl Add for Point {
    type Output = Point;
    fn add(self, other:Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

//----------------------------Default Generic Types---------------------------------//
/*
There are 2 reasons to use default generic types:
1. You want to extend a type without breaking the existing code
2. Allow customization for specific cases
*/ 

//example of the second use case: We want to add a meter and a millimeter and get a millimeter
struct Meters(u32); //this is a tuple struct
struct Millimeters(u32);
//specifying a default type for add: Check Add trait in Cargo
impl Add<Meters> for Millimeters {
    type Output = Millimeters;
    fn add(self, other:Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}
//------------------------------Same method names - different implementations------------------------------//

trait Pilot {
    fn fly(&self);
}
trait Wizard {
    fn fly();
}
struct Human;
impl Human {
    fn fly(&self) {
        println!("Waving hands furiously");
    }
}
impl Pilot for Human {
    fn fly(&self) {
        println!("flying airplane now");
    }
}
impl Wizard for Human {
    fn fly() {
        println!("Flying in broomstick");
    }
}
fn main() {
    assert_eq!(
        Point {x: 3, y: 3}, 
        Point {x: 1, y: 2} + Point {x: 2, y: 1}
    );

    let human =  Human;
    human.fly(); //executes human fly
    Pilot::fly(&human); //executes pilot fly
    
    <Human as Wizard>::fly(); //executes wizard fly; this is a new instance of the fly() method. Not part of human object

}