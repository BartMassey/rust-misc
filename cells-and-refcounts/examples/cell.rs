// https://doc.rust-lang.org/std/cell/

use cell_rc::count::Count;

use std::cell::Cell;

/// An interior-mutable counter over the restricted `Count`.
#[derive(Default)]
struct Counter(Cell<Count>);

impl Counter {

    /// Increase the count of the contained counter by one.
    /// Note that this method takes `self` by immutable
    /// reference, then changes it anyway.
    fn incr(&self) {
        let mut count = self.0.replace(Count::default());
        count.incr();
        let _ = self.0.replace(count);
    }

    /// Return the value of the contained counter.
    fn value(&self) -> u64 {
        let count = self.0.replace(Count::default());
        let value = count.value();
        let _ = self.0.replace(count);
        value
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
}
