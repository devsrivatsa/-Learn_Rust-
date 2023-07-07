fn main() {
    //enums allows us to enumerate all the variances of something
    enum IpAddrType {
        V4(String),
        V6(u8, u8, u8, u8, u8, u8)
    }

    struct IpAddr {
        kind: IpAddrType,
        address: String
    }

    //Example 2

    enum Message {
        Quit,
        Move {X:i32, y:i32},
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    impl Message {
        fn some_function() {
            println!("Hello Rust")
        }
    }

    let four = IpAddrType::V4(String::from("127.0.0.1"));
    let six = IpAddrType::V6(1,2,3,4,5,6);

    //there is no null in rust. Instead there is an option enum
    //it is included by default in the main function's scope because it is very useful
    // enum Option<T> {
    //     Some<T>,
    //     None,
    // }



    //this is how it is used
    let some_number = Some(500);
    let some_string = Some("a string");
    let absent_number:Option<i32> = None;

    let x:i8 = 5;
    let y:Option<i8> = Some(5);

    // now x is an int, y is also an int but it is of the type option enum
    // hence we cannot do operations such as addition with the 2
    // we have to first extract the value from the some variant before operating. We are also forced to handle the null case
    let sum = x + y.unwrap_or(0); //if the y is set to the None variant, the value in bracket (default value) will be used in sum

    
    
    
    //match expression
    
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
        Cooiinn(Spell)
    }
    #[derive(Debug)]
    enum Spell {
        Money,
        Leviosso,
        Flipendo,
        Stupify
    }
    
    //it allows to compare a value against a set of patterns. 
    fn value_in_cents(coin: Coin) ->u32 {
        match coin {
            Coin::Penny => {
                println!("We can do many things here. Curly braces are used if the code is long");
                100
            },
            Coin::Nickel => 5,
            Coin::Dime => 5,
            Coin::Quarter => 25,
            Coin::Cooiinn(spell) => {
                println!("You cast the {:?} spell", spell);
                10000 //this means return this
            }
        }
    }

    //combining match expression with optional enum
    fn plus_one(x:Option<i32>) -> Option<i32> {
        match x {
            _ => None, //can also be written as _ => (). this means the match will return none for anything other than a number  
            Some(i) => Some(i + 1), //this i will bind to our variable
        }
        // this is the if let syntax. this will happen if we pass in three
        if let Some(3) = x {
            println("three")
        }
    }

    

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    plus_one(3);

}