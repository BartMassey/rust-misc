use cell_rc::count::Count;


struct Message<'a> {
    note: &'static str,
    counter: &'a mut Count,
}

/// Increment the given count. Note that this function takes
/// `count` by mutable reference.
impl <'a> Message<'a> {
    fn update_count(&mut self) {
        println!("{} {}", self.note, self.counter.value());
        self.counter.incr();
    }
}

fn main() {
    let mut count = Count::default();
    let mut left = Message { note: "left", counter: &mut count };
    let mut right = Message { note: "right", counter: &mut count };
    left.update_count();
    right.update_count();
}
