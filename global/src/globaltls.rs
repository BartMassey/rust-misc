use std::cell::RefCell;

thread_local! {
    static GLOBAL: RefCell<Foo> = RefCell::new(Thing1);
}

#[derive(Debug, Clone, Copy)]
enum Foo {
    Thing1,
    Thing2,
}
use Foo::*;

fn effected() {
    GLOBAL.with(|global| {
        println!("{:?}", *global.borrow());
    });    
}

fn main() {
    GLOBAL.with(|global| {
        let mut borrowed = global.borrow_mut();
        println!("{:?}", *borrowed);
        *borrowed = Thing2;
    });
    effected();
}