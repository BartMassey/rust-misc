// https://doc.rust-lang.org/std/cell/
use cell_rc::rcmessage::*;

fn main() {
    let (m1, m2) = make_messages("m1", "m2");
    m1.println();
    m2.println();
    m1.println();
    m2.println();
}
