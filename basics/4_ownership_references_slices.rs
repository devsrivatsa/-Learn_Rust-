fn main() {
    //ownership rules:
        //
        //
        //
    
    let s1 = String::from("Hello");
    let s2 = s1; //now ownership of the value is moved to s2 from s1. s1 is no longer valid
    // println!("{}", s1); --> this will error out because value "Hello" is moved out of s1 to s2

    fn take_ownership_and_give_ownership(s:String) -> String {
        let s3 = s;
        return s3;
    }
    /*
    now the ownership is moved from s2 into s inside the function
    then from s to s3
    then from s3 to s3_2
    */
    let s3_2 = take_ownership_and_give_ownership(s2);

    //-----------------------------------------------------------------------------------------------------//
    //these are created inside the memory stack as we know the size of the variable
    let a = 1;
    let b = 2; 


    fn donothing(){
        println!("I am stored on top of the stack now")
    } 
    fn function() {
        // a stack frame in created and stored in the stack
        // now the 2 variables are stored inside stack frame
        // but c is stored in heap as string size can dynamically increase or decrease
        let a = 1;
        let b = 2;
        let c = String::from("HelloWorld");
        donothing(); //this is now stored on top of the stack until its finished executing
    }
    //-------------------------------------------references and borrowing---------------------------------------------------//
    // references dont take ownership of the underlying value

    let c = &s3_2; //now a reference of s3_2 is stored in c
    fn modify_string_without_taking_ownership(s:&String) -> &String {
        let c2 = s;
        return c2;
    }
    let c2 = modify_string_without_taking_ownership(c);
    println!("{}", c); //now there is no error because the ownership of c never changed
    println!("{}", c2); //c2 also points to c
    
    //references are immutable and therefore we cannot modify the underlying value through a reference
    let ref1 = &c; //reference
    let ref2 = &c; //mutable reference - can be used to modify unlike a non-mutable reference
    println!("maooo: {} {}", ref1, ref2); // the same variable can have many references but all have to be immutale
    // let ref3 = &mut c //this will give an error as ref3 is a mutable reference 

    let mut c3 = String::from("World");
    //scope of mutable ref begins here
    let ref3 = &mut c3; // ref3 is a mutable reference to c3. Note:only mutable variables can have mutable references
    //scope of mutable ref ends here as it is used in the above line for the last time
    
    
    let ref4 = &mut c3; //Hence we can have another mutable reference here after the scope ends here

    /*
    slices:

    slices let us reference a contiguious piece of data such as a slice from a collection
    silces are immutable references.
    */
    let demo = String::from("Ancient Aliens");
    fn get_first_four_chars(text: &String) -> &str /*str is a string literal. They are immutable.*/ 
    {
        return &text[0..4]; //or text[..4]
    }
    println!("the first 4 characters of string {} are: {}", demo, get_first_four_chars(&demo));


    //slices on array
    let arr = [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20];
    let slice1 = &arr[..5];
    let slice2 = &arr[5..10];
    let slice3 = &arr[10..arr.len()-1];

    for i in arr { 
        print!("{} ", i)
    };


}