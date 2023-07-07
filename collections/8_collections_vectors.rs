use std::vec;

fn main() {

    //vectors are stored in the heap as they can dynamically grow and shrink in size 

    let a = [1,2,3];

    let mut v1:Vec<i32> = Vec::new(); //declaring a vec of type i32
    let mut v2 = vec![1,2,3,4,5,6,7];
    let mut v55 = Vec::from(&v2);
    
    //adding elements
    v1.push(1);
    v1.push(2);
    v1.push(3);
    v1.push(4);

    //remove elements
    v1.remove(0);
    v1.remove(1);

    v1.pop(); //removes last element

    //index elements
    /*
    but this can cause run time errors.  
    This is because collections are stored in heap and there is no way to know the size of the vector during compile time.
    Hence if an index exceeding the bounds is passed in, then there will be a runtime error
    */
    let elm1 = &v2[0];
    let elm2 = &v2[1];
    let em2 = &v2[v2.len()-1];

    //hence, the following is a safe way to access elements
    let elm1 = match v2.get(1) {
        Some(first) => first, //first is just a variable in which the result is stored 
        None => &-1, //if there was a problem, then we return a ref to -1
    };

    //looping over all elements
    for i in &v2 {
        println!("{}", i);
    }
    //editing the elements
    for i in &mut v2 {
        *i += 50 //the star here is a dereferencing operator
    }
    for i in v2 {
        print!("{i} ");
    }

    for i in 10..0 {
        println!("reverse numbers");
        print!("{} ", i);  
    }
    
    // getting the index of a specific element (7)
    let iv = vec![1,2,3,4,5,6,7];
    let idx_of_7 = match iv.iter().position(|&elm| elm == 7) {
        some(first) => first,
        None => &-1
    }


    v1.clear()

    //storing differen type of data through - enum variance
    enum Data {
        Int(i32),
        Float(f64),
        Text(String)
    }
    let row = vec![
        Data::Int(50),
        Data::Int(55),
        Data::Int(70),
        Data::Float(99.99),
        Data::Float(9.0),
        Data::Float(0.8),
        Data::Text(String::from("Hello")),
        Data::Text(String::from("Tesla"))
    ];


}