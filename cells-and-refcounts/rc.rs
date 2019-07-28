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

/// Increment the given counter. Note that this function
/// takes `counter` by immutable reference.
fn update_counter(counter: &Counter) {
    counter.incr();
}

fn main() {
    let counter1 = Counter::default();
    let counter2 = counter1.clone();
    println!("{}", counter1.value());
    update_counter(&counter2);
    println!("{}", counter1.value());
}
