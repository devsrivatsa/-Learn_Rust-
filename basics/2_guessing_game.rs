use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guessing game");
    let secret_num = rand::thread_rng().gen_range(1, 101);
    loop {
        println!("Enter a number");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
    
        let guess:u32 = match guess.trim().parse() {
            Ok(secret_num) => secret_num,
            Err(_) => continue,
        };
    
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("You Win!!!");
                break;
            }
    
        }
        
        println!("You guessed : {}", guess);
    }
}
