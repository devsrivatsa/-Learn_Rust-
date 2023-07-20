use std::sync::Mutex; //mutex stands for mutual exclusion
use std::sync::Arc;
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(5)); //now counter can be shared with different threads;
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    } 
    println!("Result: {}", *counter.lock().unwrap());
}