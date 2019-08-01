use stacktrait::*;

use std::collections::LinkedList;

fn main() {
    let mut s = Vec::new();
    s.spush(&5);
    println!("{}", s.spop().unwrap());

    let mut s = LinkedList::new();
    s.spush("hello");
    println!("{}", s.spop().unwrap());
}
