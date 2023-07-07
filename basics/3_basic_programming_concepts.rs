fn main() {
    
    // 1 VARIABLES

    let x = 5; //all variables are immuatble
    let mut x2 = 23; // adding mut will make the variable mutable --> value can be changed; type can be changed;
    x2 = 24;
    let mut x2:&str = "Hello_world";

    /* this variable can never be changed, 
    can never be redelared as mut, 
    cannot be returned in a function */
    const CONSTANT_VARIABLE:u32 = 100;
    //-------------------------------------------------------------------------------------------------------------//
    
    //2. DATA TYPES - simple and compound
    
    //-----------------------simple---------------------------//
    
    // integers
    let a= 12; //int - signed can be 8, 16, 32, 64, 128
    let b = 12_8; //decimal
    let c = 0o77; //octal
    let d = 0b1111_0000; //binary
    let e = b'A'; //byte
    let f:u8 = 255; // 8 bit unsigned - can be 16, 32, 64, 128

    // floating points
    let f = 2.0; //default is 64
    let g: f32 = 2.0; //32 bit floating point

    //boolean
    let h:bool = false;
    let i =  true;

    // character
    let j = 'z';
    
    //-----------------------compound---------------------------//

    //tuples
    let tup = ("Rust", 100_000); //declaration
    let (lang, awesomeness) = tup; //destructuring
    let fake_count = tup.1; //indexing tuples

    //fixed size arrays - immutable
    let arr = [1,2,3,4,5]; // declare/create array
    let arr_5_zeros = [0; 5]; // create array of 5 elements filled with 0
    //--------indexing--------//
    arr_5_zeros[0];
    arr[2];
    arr[4];
    
    //-------------------------------------------------------------------------------------------------------------//

    //3. FUNCTIONS

    fn sum_mul(x:i32, y:i32, e:i32) -> i32 {
        println!("this is a function");
        return (x+y)*2;
    }
    sum_mul(2,3,2); //calling a function

    //-------------------------------------------------------------------------------------------------------------//

    //4. CONTROL FLOW
    
    //basic if statement -- wrapping brackets around condition is optional
    let guess = 5;
    if guess == 5 {
        println!("bam!!")
    } else if guess > 5 {
        println!("Double Bam!!")
    } else {
        print!("small bam..")
    }
    //if expression
    let cool = if (guess==5) {"Bam"} else {"small bam"};

    //-------------------------------------------------------------------------------------------------------------//

    //4. LOOPS
    let mut c=0; 
    loop {
        if c == 10 {
            break;
        } else {
            println!("{}", c);
            c+=1;
        }
     }
    // loop as expression
    c = 0;
    let mut r = loop {
        if c==10 {
            break c //adding c after break will return c
        }
        c += 1;
    };
    
    // while loop
    c = 0;
    r = 10;
    while c != 100 {
        if c == 50 {
            continue;
        }
        c += r;
    }

    // for loop - with range
    c = 0;
    r = 10;
    for i in 0..100 {
        r += i;
    }

    // foreach loop
    c = 0;
    r = 10;
    let demolist = [1; 80];
    for i in demolist {
        r += i;
    }

}