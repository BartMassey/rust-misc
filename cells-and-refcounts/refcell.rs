// https://doc.rust-lang.org/std/cell/
mod count;
use count::Count;

use std::cell::RefCell;

/// An interior-mutable counter over the restricted `Count`.
#[derive(Default)]
struct Counter(RefCell<Count>);

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

    /// Increase the count of the contained counter by one,
    /// and return the increased count.  Note that this
    /// method takes `self` by immutable reference, then
    /// changes it anyway.
    fn incr_value(&self) -> u64 {
        let mut count1 = self.0.borrow_mut();
        // XXX No! A RefCell follows the borrow rules at
        // runtime, so this will panic.
        let count2 = self.0.borrow();
        count1.incr();
        count2.value()
    }
}

/// Increment the given counter. Note that this function
/// takes `counter` by immutable reference.
fn update_counter(counter: &Counter) {
    counter.incr();
}

fn main() {
    let counter = Counter::default();
    println!("{}", counter.value());
    update_counter(&counter);
    println!("{}", counter.value());
    println!("{}", counter.incr_value());
}
