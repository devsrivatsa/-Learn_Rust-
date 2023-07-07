use std::time::Duration;
use tokio::time::sleep;

 //tokio is a polling runtime that allows the main function to be async
#[tokio::main] //by default tokio uses a threadpool to execute tasks
// #[tokio::main(flavor="current_thread")] //-->uses time slicing instead of threadpool
async fn main() {

    //basic way to work with async
    // let f = my_function(1);
    // println!("futures are lazy as they are a 0 cost abstractions; they are also lazy as we can cancell the future");
    // f.await;

    //using tokio tasks --> similar to goroutines
    let mut handles =  vec![];
    for i in 0..2 {
        let handle = tokio::spawn(async move {
            my_function(i).await;
        });
        handles.push(handle);
    }

    for h in handles {
        h.await.unwrap(); //returns a Result enum 
    }

}



async fn my_function(i: i32) {
    println!("[{i}] I am an async function!");
    let s1 = read_from_db().await; //await can only be called from within an async function
    println!("[{i}] first result: {s1}");
    let s2 = read_from_db().await;
    println!("[{i}] second result: {s2}");
}

async fn read_from_db() -> String {
    sleep(Duration::from_millis(50)).await;
    String::from("lets get rusty!!!")
}