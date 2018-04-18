// Copyright © 2018 Bart Massey
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

#[allow(dead_code)]

enum Color {
    Red,
    Green,
    Blue,
    Custom(f64, f64, f64),
}

fn main() {
    let c = Color::Custom(0.7,0.5,0.3);
    match c {
        Color::Red => panic!("saw red"),
        Color::Custom(r, _, _) =>
            if r > 0.0 { panic!("saw some red") },
        _ => println!("whew!")
    }
}
