fn main() {
    //match statement should be exhaustive
    #[derive(Debug)]
    enum Lang {
        English,
        Spanish,
        Japanese,
        French,
    }
    let language = Lang::English;

    match language {
        Lang::English => println!("Hello World"),
        Lang::French => println!("J'parle Francies"),
        Lang::Japanese => println!("Hello World sama"),
        Lang::Spanish => println!("Hello World spanish"),
        _ => println!("Unsuported language!")
    }

    //use the unsupported input in the msg 
    match language {
        Lang::English => println!("Hello World"),
        Lang::French => println!("J'parle Francies"),
        Lang::Japanese => println!("Hello World sama"),
        Lang::Spanish => println!("Hello World spanish"),
        x => println!("Unsuported language: {:?}", x)
    }

    //if let pattern matching
    if let x = 5 {
        println!("{}", x);
    }

    //while let conditional loops
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // for loops enumeration pattern matching
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    //let statements pattern matching
    let (x,y,z) = (1,2,4);

    //function pattern matching
    fn print_coordinates(&(x,y): &(i32, i32)) {
        println!("Current Location: ({}, {})", x, y);
    }
    let point = (3,5);
    print_coordinates(&point);

    //refutable patterns might not always match + if let statements
    let x: Option<&str> = None;
    if let Some(x) = x {
        println!("{}", x);
    }

}