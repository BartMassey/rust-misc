// Copyright © 2018 Bart Massey
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

// Demonstrate "flexible" operator overloading.

#![allow(unused)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}
use Suit::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Rank {
    Num(u32),
    Jack,
    Queen,
    King,
    Ace,
}
use Rank::*;

impl Rank {
    fn value(self) -> u32 {
        match self {
            Num(n) => n,
            Jack => 11,
            Queen => 12,
            King => 13,
            Ace => 1,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq)]
struct Card {
    rank: Rank,
    suit: Suit,
}

impl PartialEq<Card> for Card {
    fn eq(&self, other: &Card) -> bool {
        self.rank == other.rank
    }
}
    
impl PartialEq<u32> for Card {
    fn eq(&self, other: &u32) -> bool {
        self.rank.value() == *other
    }
}

impl PartialEq<Card> for u32 {
    fn eq(&self, other: &Card) -> bool {
        *other == *self
    }
}

fn compare_test() {
    let jc = Card{ rank: Jack, suit: Clubs };
    let jh = Card{ rank: Jack, suit: Hearts };
    let th = Card{ rank: Num(10), suit: Hearts };
    assert_eq!(jc, jc);
    assert_eq!(jc, jh);
    assert_eq!(jc, 11);
    assert_ne!(th, 11);
    assert_eq!(11, jc);
    assert_ne!(11, th);
}

#[test]
fn call_compare_test() {
    compare_test();
}

fn main() {
    compare_test();
}
