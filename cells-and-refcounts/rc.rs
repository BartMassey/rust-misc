// https://doc.rust-lang.org/std/cell/
mod count;
use count::Count;

use std::cell::RefCell;
use std::rc::Rc;

/// An interior-mutable shared counter over the restricted `Count`.
#[derive(Default, Clone)]
struct Counter(Rc<RefCell<Count>>);

impl Counter {

    /// Increase the count of the contained counter by one.
    /// Note that this method takes `self` by immutable
    /// reference, then changes it anyway.
    fn incr(&self) {
        let mut count = self.0.borrow_mut();
        count.incr();
    }

    /// Return the value of the contained counter.
    fn value(&self) -> u64 {
        let count = self.0.borrow();
        count.value()
    }

}

/// Message to print plus usage counter.
struct Message {
    message: &'static str,
    counter: Counter,
}

impl Message {
    fn new(message: &'static str, counter: Counter) -> Message
    {
        Message { message, counter }
    }

    fn print(&self) {
        println!("{} {}", self.message, self.counter.value());
        self.counter.incr();
    }
}

fn main() {
    let counter = Counter::default();
    let m1 = Message::new("m1", counter.clone());
    let m2 = Message::new("m2", counter);
    m1.print();
    m2.print();
    m1.print();
    m2.print();
}
