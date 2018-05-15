// Copyright © 2018 Bart Massey
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

use std::collections::HashMap;
use std::hash::Hash;

fn histogram<I, T>(values: I) -> HashMap<T, usize>
    where I: IntoIterator<Item=T>, T: Hash + Eq
{
    let mut h = HashMap::new();
    for k in values {
        h.entry(k)
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }
    h
}

fn main() {
    let alphas = "hello world"
        .chars()
        .filter(|c| c.is_alphabetic());
    let h = histogram(alphas);
    let mut keys: Vec<&char> = h.keys().collect();
    keys.sort();
    for key in keys {
        println!("{}: {}", key, h[key]);
    }
}
