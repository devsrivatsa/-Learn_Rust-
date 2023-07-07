//RefCell enforces the borrowing rules at runtime -> this is unsafe. If you break borrowing rules at runtime, the program will panic

//Interior mutability is a design pattern in rust which allows to mutate data even when there are immutable refs to that value, which is notmally disallowed by the borrowing rules.    
//Interior Mutability: Because RefCell<T> allows immutable or mutable borrows at runtime, you can mutate a value inside RefCell<T> even when RefCell<T> is immutable. But this is not possible in box.

//Box enforces the borrowing rules at compile time -> this means errors will be caught sooner and in runtime there will not be any performance costs

//ref cell can only be used in single threaded program

use::std::cell::RefCell;
use::std::rc::Rc;

pub trait Messenger {
    fn send_msg(&self, msg: &str);
}
pub struct LimitTracker<'a, T:Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize
}

impl<'a, T> LimitTracker<'a, T> where T: Messenger {
    pub fn new(messenger: &T, max:usize) -> LimitTracker<T> {
        LimitTracker { messenger, value: 0, max }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percenage_of_max = self.value as f64 / self.max as f64;
        if percenage_of_max >= 1.0 {
            self.messenger.send_msg("Error: You are over your quota");
        } else if percenage_of_max >= 0.9 {
            self.messenger.send_msg("Urgent warning: You have used up over 90% of quota");
        } else if percenage_of_max >= 0.75 {
            self.messenger.send_msg("Warning: You have used over 75% of your quota");
        }
    }
}

////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {
    use super::*; //brings everything from parent module into scope
    
    pub struct MockMessenger {
        //sent_msgs: Vec<String>
        sent_msgs: RefCell<Vec<String>>
    }
    impl MockMessenger {
        fn new() -> MockMessenger {
            return MockMessenger { 
                //sent_msgs: vec![] --> insted of doing it this way
                sent_msgs: RefCell::new(vec![]), 
            };
        }
    }
    impl Messenger for MockMessenger {
        fn send_msg(&self, message: &str) {
            //we are using the borrow_mut() method on the immutable reference of self to mutate a value inside
            //the borrow_mut is a method of ref cell pointer that allows access to underlying value (the vector) as a mutble element
            //note that we are accessing this element through an immutable reference to self (&self)
            /*
            If we do not use borrow_mut, we would get an error because we are accessing a value from an immutable reference
            Also, we cannot change the reference to self to mutable because the messenger trait requires that we have an immutable reference to self.
            Hence, in order resolve this predicament, we use borrow_mut()
            */ 
            self.sent_msgs.borrow_mut().push(String::from(message));
        }
    }
    
    #[test]
    fn it_sends_an_over_75_percent_warning_msg() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
        limit_tracker.set_value(80);
        //we do not want a mutable reference here, hence we use borrow() to get an immutable reference
        assert_eq!(mock_messenger.sent_msgs.borrow().len(), 1);
    }
}

//combining the Rc with RefCell will allow multiple owners with mutable data

fn main() {

}