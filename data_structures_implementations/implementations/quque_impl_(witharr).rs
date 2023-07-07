struct CircularQueue {
    head:i32,
    tail:i32,
    size:i32,
    capacity:i32,
    queue: Vec<i32>
}

impl CircularQueue {
    fn new(c:i32) -> CircularQueue {
        return CircularQueue { head:0, tail: 0, size: 0, capacity: c, queue: Vec::new()}
    }

    fn enqueue(&mut self, elm:i32) -> bool {
        if self.is_full() {
            return false
        }
        self.queue.insert(self.tail as usize, elm);
        self.tail = (self.tail + 1)%self.capacity;
        self.size += 1;
        return true
    }
    
    fn dequeue(&mut self) -> i32 {
        if !(self.is_empty()) {
            let ret = self.queue.remove(self.head as usize);
            self.queue.insert(self.head as usize, 0);
            self.head = (self.head + 1) % self.capacity;
            self.size -= 1;
            return ret;
        }
        return -1000
    }
    

    fn is_full(&self) -> bool {
        self.size == self.capacity
    }
    
    fn is_empty(&self) -> bool {
        self.size == 0
    }
    
    fn top(&self) -> Result<&i32, &str> {
        if !(self.is_empty()) {
            return Ok(&self.queue[self.head as usize]);
        } else {
            return Err("The queue is empty!");
        }
        
    }

    fn rear(&self) -> Result<&i32, &str> {
        if !(self.is_empty()) {
            let rear_idx = (self.head + self.size -1) % self.capacity;
            return Ok(&self.queue[rear_idx as usize]);
        } else {
            return Err("The query is empty!");
        }
    }
    
    fn print_queue(&mut self) {
        println!("Queue --> {:?}", self.queue);
    }


}

fn main() {
    let mut q = CircularQueue::new(5);
    q.enqueue(100);
    q.enqueue(200);
    q.enqueue(300);
    q.enqueue(400);
    q.enqueue(500);
    q.print_queue();
    
    let mut top = q.top();
    match top {
        Ok(value) => println!("head: {}",value),
        Err(msg) => println!("{}", msg)
    }

    let mut rear = q.rear();
    match rear {
        Ok(value) => println!("tail: {}", value),
        Err(msg) => println!("{}", msg)
    }

    println!("Dequeued: {}", q.dequeue());
    println!("Dequeued: {}", q.dequeue());
    println!("Dequeued: {}", q.dequeue());
    q.print_queue();

    let mut top = q.top();
    match top {
        Ok(value) => println!("head: {}",value),
        Err(msg) => println!("{}", msg)
    }

    let mut rear = q.rear();
    match rear {
        Ok(value) => println!("tail: {}", value),
        Err(msg) => println!("{}", msg)
    }

    q.enqueue(6000);
    q.enqueue(7000);

    q.print_queue();
    let mut top = q.top();
    match top {
        Ok(value) => println!("head: {}",value),
        Err(msg) => println!("{}", msg)
    }

    let mut rear = q.rear();
    match rear {
        Ok(value) => println!("tail: {}", value),
        Err(msg) => println!("{}", msg)
    }

}