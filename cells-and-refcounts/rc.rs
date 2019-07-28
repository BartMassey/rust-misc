// https://doc.rust-lang.org/std/cell/
mod count;
mod rcmessage;
use rcmessage::*;

fn main() {
    let (m1, m2) = make_messages("m1", "m2");
    m1.print();
    m2.print();
    m1.print();
    m2.print();
}
