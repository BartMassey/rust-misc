use lazy_static::*;
use std::sync::Mutex;

lazy_static! {
    static ref GLOBAL: Mutex<Foo> = Mutex::new(Thing1);
}

#[derive(Debug, Clone, Copy)]
enum Foo {
    Thing1,
    Thing2,
}
use Foo::*;

fn effected() {
    let global = GLOBAL.lock().unwrap();
    println!("{:?}", *global);    
}

fn main() {
    let mut global = GLOBAL.lock().unwrap();
    println!("{:?}", *global);
    *global = Thing2;
    // Note that this drop is crucial; otherwise
    // effected() will try to lock the Mutex
    // a second time, deadlocking the program.
    //
    // You could also just use a block around the
    // previous lines to get the compiler to auto-drop.
    drop(global);
    effected();
}