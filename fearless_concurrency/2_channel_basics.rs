use std::sync::mpsc; //multiple producer, single consumer
use std::{thread, vec};
use std::time::Duration;

fn sending_and_receiving_one_value() {

    let (tx, rx) = mpsc::channel(); 
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap() //the ownership of val is transferred here to another thread
    });
    let received = rx.recv().unwrap();
    println!("Got: {}", received);

    //val2 can be modified even after transmitting because it implements copy trait!!! 
    let (tx2, rx2) = mpsc::channel();
    thread::spawn(move || {
        let val2 = 35;
        tx2.send(val2).unwrap();
        println!("val is {}", val2);
    });
    let received2 = rx2.recv().unwrap() + 29;
    println!("Got: {}", received2);
}

fn sending_and_receiving_multiple_values() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let vals = vec![
            String::from("hello"),
            String::from("from"),
            String::from("the"),
            String::from("other"),
            String::from("side"),
        ];
        for s in vals {
            tx.send(s).unwrap();
            thread::sleep(Duration::from_millis(2));
        }
    });

    for received in rx {
        print!("{} ", received);
    }
}

fn multiple_producers_single_receiver() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    
    //thread 1
    thread::spawn(move || {
        let v = vec![
            String::from("Mr"),
            String::from("Bean"),
            String::from("is"),
            String::from("very"),
            String::from("happy"),
        ];
        for s in v {
            tx.send(s).unwrap();
            thread::sleep(Duration::from_millis(2));
        }
    });

    //thread 2
    thread::spawn(move || {
        let v2 = vec![
            String::from("The"),
            String::from("cat"),
            String::from("is"),
            String::from("alone"),
        ];
        for s in v2 {
            tx1.send(s).unwrap();
            thread::sleep(Duration::from_millis(2));
        }
    });

    //receive from both threads
    for received in rx {
        print!("{} ", received);
    }
}

fn main() {
    sending_and_receiving_one_value();
    sending_and_receiving_multiple_values();
    println!();
    multiple_producers_single_receiver();
}

