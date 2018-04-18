// Copyright © 2018 Bart Massey
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

fn main() {
    let mut a = [1,2,3];
    let t = a[0];
    a[0] = a[2];
    a[2] = t;
    println!("{:?}", a);
}
