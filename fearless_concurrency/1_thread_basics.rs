use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    handle.join().unwrap(); //this ensures that the spawned threads are complete before proceeding forward
    for i in 1..5 {
        println!("Hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    } //without a join, there is no gurantee that the spawned threads will ever complete. 
    
    //handle.join().unwrap(); //this endures that the main thread does not close before all the spawned jobs are complete.

    //the closures used in threads should always be move closurs as the main thread can end before the spawned thread and end the lifetimes of variables
    let v = vec![1,2,3,4,5];
    thread::spawn(move || { println!("value is moved inside even if only its immutable ref is used in println: {:?}", v); }).join().unwrap();   
}