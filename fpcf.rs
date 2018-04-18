// Copyright © 2018 Bart Massey
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

use std::str::FromStr;
use std::env::Args;

fn next_f64(args: &mut Args) -> f64 {
    f64::from_str(&args.next().unwrap()).unwrap()
}

fn main() {
    let mut args = std::env::args();
    let _ = args.next();
    let x = next_f64(&mut args);
    let y = next_f64(&mut args);
    println!("{:?}", x < y);
}
