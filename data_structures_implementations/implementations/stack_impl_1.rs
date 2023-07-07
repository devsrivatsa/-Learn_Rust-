pub struct StackWithArray {
    data:Vec<i32>,
    size: i32,
    top: i32,
}
impl StackWithArray {
    fn new(s:i32) -> Self {
        StackWithArray {
            data: Vec::new(),
            size: s,
            top: -1
        }
    }
    fn empty(&mut self) -> bool {
        self.top == -1
    }
    fn full(&mut self) -> bool {
        self.top+1 == self.size
    }
    fn current_size(&mut self) -> i32 {
        self.top() + 1
    }
    
    fn push(&mut self, item:i32) {
        if !self.full() {
            self.top+=1;
            self.data.push(item);
        }
    }
    fn pop(&mut self) -> i32 {
        let mut popped = -1;
        if !self.empty() {
            popped = self.data.pop().unwrap();
            self.top -= 1;
        }
        popped
    }
    fn top(&mut self) -> &i32 {&self.top}

    fn print_stack(&mut self) {
        print!("Current Stack Elements: ");
        for i in 0..self.data.len() {
            print!("{} ", &self.data[i]);
        }
        println!();
    }

}

fn main() {
    let mut st = StackWithArray::new(10);
    st.push(25);
    st.push(50);
    st.push(75);

    for i in 0..5 {
        st.push(i);
    }
    println!("current size: {}", st.current_size());
    println!("top: {}", st.top());
    st.print_stack();

    st.pop();
    st.pop();
    println!("last popped: {}", st.pop());
    st.print_stack();
    println!("current size: {}", st.current_size());
    println!("top: {}", st.top());
}