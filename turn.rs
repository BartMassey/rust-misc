// Copyright © 2018 Bart Massey
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

#[derive(Debug)]
enum Turn {
    Left,
    Right,
}

impl <'a> From<&'a str> for Turn {
    fn from(s: &'a str) -> Turn {
        match s {
            "L" => Turn::Left,
            "R" => Turn::Right,
            t => panic!("bad turn description {}", t),
        }
    }
}

fn main() {
    println!("{:?}", Turn::from("L"));
}
