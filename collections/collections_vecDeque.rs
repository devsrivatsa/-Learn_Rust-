use std::collections::VecDeque;

fn main() {
    //1. creating a queue/stack
    let mut queue:VecDeque<i32> = VecDeque::new();
    let mut stack:VecDeque<i32> = VecDeque::new();

    
    //adding elements to the back
    queue.push_back(10);
    queue.push_back(20);
    queue.push_back(30);
    println!("Queue: {:?}", queue);

    //adding elements in the front
    stack.push_front(100);
    stack.push_front(200);
    stack.push_front(300);
    stack.push_front(400);
    println!("Stack: {:?}", stack);


    //remove elements from front
    println!("Dequeued elm: {}", queue.pop_front().unwrap());
    println!("Dequeued elm: {}", queue.pop_front().unwrap());
    println!("Queue: {:?}", queue);

    //remove elements from back
    println!("unstack elm: {}", stack.pop_back().unwrap());
    println!("unstack elm: {}", stack.pop_back().unwrap());
    println!("unstack elm: {}", stack.pop_back().unwrap());
    println!("Stack: {:?}", stack);

    //check if an element is present in the deque
    println!("Does stack contains 400 ? => {}", stack.contains(&400));

    //check if the queue/stack is empty
    queue.is_empty();
    stack.is_empty();


}