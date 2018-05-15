// Copyright © 2018 Bart Massey
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

use std::collections::HashMap;
use std::hash::Hash;
use std::iter::FromIterator;
use std::fmt;

#[derive(Clone, Debug)]
struct Histogram<T: Hash + Eq>(HashMap<T, usize>);

impl<T: Hash + Eq> Histogram<T> {

    fn histogram<I>(values: I) -> Self
        where I: IntoIterator<Item=T>
    {
        let mut h = HashMap::new();
        for k in values {
            h.entry(k)
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }
        Histogram(h)
    }

}

impl<T: Hash + Eq> FromIterator<T> for Histogram<T> {
    fn from_iter<I>(iter: I) -> Self
        where I: IntoIterator<Item=T>
    {
        Histogram::histogram(iter)
    }
}

impl<T> fmt::Display for Histogram<T>
    where T: Hash + Eq + Ord + fmt::Debug
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        if self.0.is_empty() {
            return write!(fmt, "Histogram {{}}");
        }
        let mut keys: Vec<&T> = self.0.keys().collect();
        let mut items = Vec::new();
        keys.sort();
        for key in keys {
            items.push(format!("{:?}: {}", key, self.0[key]));
        }
        write!(fmt, "Histogram {{{}}}", items.join(", "))
    }
}

fn main() {
    let h: Histogram<char> = "hello world"
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect();
    println!("{}", h);

    let words: Histogram<_> = "a banana is a banana is a banana"
        .split_whitespace()
        .collect();
    println!("{}", words);
}
