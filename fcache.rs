use std::hash::Hash;
use std::collections::HashMap;

/// The Fibonacci function, implemented the slow way.
fn fib(n: u64) -> u64 {
    if n < 2 {
        return 1;
    }
    fib(n - 1) + fib(n - 2)
}

struct Memo<T, U>(HashMap<T, U>);

impl<T: Hash + Eq + Clone, U: Clone> Memo<T, U> {
    fn new() -> Memo<T, U> {
        Memo(HashMap::new())
    }

    fn eval(&mut self, f: & dyn Fn(&mut Memo<T, U>, T)->U, t: T) -> U {
        if let Some(u) = self.0.get(&t) {
            return u.clone();
        }
        let u = f(self, t.clone());
        self.0.insert(t, u.clone());
        u
    }
}

/// The Fibonacci function, with memoization.
fn fibm(m: &mut Memo<u64, u64>, n: u64) -> u64 {
    if n < 2 {
        return 1;
    }
    m.eval(&fibm, n - 1) + m.eval(&fibm, n - 2)
}

fn main() {
    let mut args = std::env::args().skip(1);
    let mode = args.next().unwrap();
    let n: u64 = args.next().unwrap().parse().unwrap();

    let f = match &*mode {
        "plain" => fib(n),
        "memo" => fibm(&mut Memo::new(), n),
        mode => panic!("unknown mode {}", mode),
    };
    println!("{}", f);
}
    
    
