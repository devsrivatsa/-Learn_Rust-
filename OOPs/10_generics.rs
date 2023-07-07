//generic function that will return the largest num in an array
//this is only for numeric types
fn get_largest<T: PartialOrd + Copy>(some_list: Vec<T>) -> T {
    let mut largest = some_list[0];
    for n in some_list {
        if n > largest {
            largest = n;
        }
    }
    return largest;
}

struct Point<T> {
    x:T,
    y:T
}
//generic methods on the point struct
impl<T> Point<T> {
    fn x(&self) -> &T {
       &self.x 
    }
}
// this is only for points that have 64 bit floating point numbers
impl<f64> Point<f64> {
    fn y(&self) -> &f64 {
        &self.y
    }
}

//point with multiple types
struct Point<T, U> {
    x: T,
    y: U
}
// an implementation on the point<T, U> which will mixup this point and another point
impl<T,U> Point<T, U> {
    fn mixup(self, other:Point<A, B>) -> Point<T, B> 
    {
        Point {
            x: self.x,
            y: other.y
        }
    }
}



// there is also another useful enum - Result
// enum Result<T, E> {
//     Ok(T),
//     Err(E)
// }

//this is the generic option enum
// enum Option<T> {
//     Some(T),
//     None,
// }


fn main() {
    let p1 = Point{x:1,y:2};
    let p2 = Point{x:1.12, y:5.32};

    p1.x();; //x() is avaliable for p1
    //but both x() and y() are available for p2.
    p2.x();
    p2.y();

    let p3 = p2.mixup(Point {x:"Hello", y:007}); // now the results here will be a mixup of p2 and the new point




    let p3 = PointType2{x:1, y:1.34};
}
