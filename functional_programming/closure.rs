// closures are anonymous functions
fn main() {
    //basic closure 
    //the type of the input arguments are implicitely inferred by compiler.
    //But if a different type of argument is passed when calling the closure again, this will produce an error.
    //Hence the first type passed will be the concrete type of the input parameter

    let grtst = |num1, num2| { 
        if num1 > num2 {num1} else {num2}
    };
    //one liner function
    let one_line_closure = || "Confringo!";


    //structs and enums using a closure will have to 1. be generic, 2.implement a trait bound
    
    struct Cacher<T>
    where T: Fn(u32) -> u32 //this is the type [fn is a trait]
    {
        calculation: T,
        value: Option<u32> //this will be initially None. then it will contain the result of calculation after performing the calculation
    }
    impl<T> Cacher<T>
    where T: Fn(u32) -> u32
    {
        fn new(calculation: T) -> Cacher<T> {
            cacher {
                calculation,
                value: None
            }
        }
        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = self.calculation(arg);
                    self.value = Some(v);
                    v
                }
            }
        }
    }



    println!("{:?}", grtst(1,2));

    println!("{:?}", one_line_closure())
}