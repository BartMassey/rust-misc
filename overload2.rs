// Copyright © 2018 Bart Massey
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

// Demonstrate overloading Add for a couple of types.
// Code taken essentially straight from the library
// documentation.

use std::ops::Add;

#[derive(Clone, Copy, Debug)]
struct Type1(usize);

#[derive(Clone, Copy, Debug)]
struct Type2(usize);

impl Add for Type1 {
    type Output = Type1;
    fn add(self, other: Type1) -> Type1 {
        Type1(self.0 + other.0)
    }
}

impl Add for Type2 {
    type Output = Type2;
    fn add(self, other: Type2) -> Type2 {
        Type2(self.0 + other.0)
    }
}

fn main() {
    println!("{:?}", Type1(1) + Type1(1));
    println!("{:?}", Type2(1) + Type2(1));
}
