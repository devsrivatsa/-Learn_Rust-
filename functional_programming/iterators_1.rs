//iterators allows the values stored in a data structure to be iterated regardless of the order.
//iterator is a trait. They are lazy. hence they need to be consumed

/* structs implementing iterator should 
    1. implementable method called next: next needs a mutable reference to self
    2. define a type

    pub trait Iterator {
        type Item; ////this is an associated type. i.e it can be any type which we will only know at runtime
        fn next(&mut self) -> Option<self::Item>;
    }
*/

fn main() {
    let v1 = vec![1,2,3,4,5];
    let v1_iter = v1.iter();

    //the loop takes ownership of v1_iter and makes it mutable behind the sceen
    for value in v1_iter {
        print!("Got {}", value);
    }

    let v2 = vec![11,12,13,14,15,16,17,18];
    let mut v2_iter = v2.iter(); //needs to be mutable as we will call next on v2_iter until the end of the vector
    
    print!("using next method:  "); // itrator returns immutable references for valus inside v2
    for i in 0..7 {
        let t = v2_iter.next().unwrap(); //enum returns an option. Hence we need to unwrap it
        println!("{}", t);
    }

    let mut v3 = vec![1,2,34];
    let v3_iter_mut = v3.iter_mut(); //to iterate mutable referenes

    /*
    iter() --> for immutable references
    iter_mut() --> for mutable references
    into_iter() --> for owned types
    */



    // iterator - sum
    let v4 = vec![1,2,4,8,16,32,64,128,256,512,1024];
    let sum_v4:i32 = v4.iter().sum();
    println!("The sum of v4 is {}", sum_v4);
    
    //adaptor method
    
    //map --> returns an iterator. Hence a consumer method must be used -> collect() [to transform result into a collection]
     let map_example_1:Vec<i32> = v1.iter().map(|x| x*2).collect();
     

    #[derive(PartialEq, Debug)]
    struct Shoe {
        size: u32,
        style: String
    }
    fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size:u32) -> Vec<Shoe> {
        //filter returns another iterator
        shoes.into_iter().filter(|s| s.size == shoe_size).collect() 
    }


     // chaining map expressions
     let map_example_2 = v1.iter().map(|x| x+1).map(|x| x*2).map(|x| x/3);

     //filter
     let result:Vec<i32> = map_example_2.filter(|x| x%2 == 0).collect();
     result.iter().for_each(|f| print!("{} ", f));

    //chaining map, filter and foreach expressions
     
    
}
//...................................................//creating an iterator...........................................................................

struct Counter {
    count: u32,
}
impl Counter {
    fn new() -> Counter {
        Counter {count: 0}
    }
}
impl Iterator for Counter {
    type Item = u32; //1.specify type

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        }
        else {
            None
        }
    }
}
#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

#[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new().zip(  //zip combines 2 iterators like python's zip and returns another iterator
        Counter::new().skip(1) //skip is an adapter method that skips the first n elements and return another iterator
    ).map(|(a, b)| a*b) //multiply elements from both iterator
    .filter(|a| a%3 == 0) //filters elements divisible by 3
    .sum(); // sums the result
}

// let mut counter = Counter::new();
// for _ in 0..5 {
//     let mut k = 
// }



   