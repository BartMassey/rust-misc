pub use crate::count::Count;

pub use std::cell::RefCell;
pub use std::rc::Rc;

/// An interior-mutable shared counter over the restricted `Count`.
#[derive(Default, Clone)]
pub struct Counter(Rc<RefCell<Count>>);

impl Counter {

    /// Increase the count of the contained counter by one.
    /// Note that this method takes `self` by immutable
    /// reference, then changes it anyway.
    pub fn incr(&self) {
        let mut count = self.0.borrow_mut();
        count.incr();
    }

    /// Return the value of the contained counter.
    pub fn value(&self) -> u64 {
        let count = self.0.borrow();
        count.value()
    }

}

/// Message to print plus usage counter.
#[derive(Clone)]
pub struct Message {
    pub message: &'static str,
    pub counter: Counter,
}

impl Message {

    pub fn new(message: &'static str, counter: Counter) -> Message
    {
        Message { message, counter }
    }

    /// Print the message and counter.
    pub fn print(&self) {
        print!("{} {}", self.message, self.counter.value());
        self.counter.incr();
    }

    /// Add a newline.
    #[allow(unused)]
    pub fn println(&self) {
        self.print();
        println!();
    }
}

/// Make a couple of messages with given message strings.
pub fn make_messages(m1: &'static str, m2: &'static str)
                 -> (Message, Message)
{
    let counter = Counter::default();
    let m1 = Message::new(m1, counter.clone());
    let m2 = Message::new(m2, counter);
    (m1, m2)
}
