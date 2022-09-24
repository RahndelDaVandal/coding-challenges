// This problem was asked by Amazon.
//
// There exists a staircase with N steps, and you can climb up either 1 or 2 steps at a time.
// Given N, write a function that returns the number of unique ways you can climb the staircase.
// The order of the steps matters.
//
// For example, if N is 4, then there are 5 unique ways:
// 1, 1, 1, 1
// 2, 1, 1
// 1, 2, 1
// 1, 1, 2
// 2, 2
//
// What if, instead of being able to climb 1 or 2 steps at a time, you could climb any number
// from a set of positive integers X? For example, if X = {1, 3, 5},
// you could climb 1, 3, or 5 steps at a time.

use std::fmt;

fn main() {
    find_permutations(4);
}

fn find_permutations(num_of_stairs: i32) {
    let mut p = Permutation::new(num_of_stairs, 2);
    println!("{p}");
    for _ in 0..2 {
        if !p.is_valid() {
        }
    }
}

pub struct Permutation {
    values: Vec<i32>,
    idx: usize,
    num_of_stairs: i32,
    max_step: i32,
}
impl Permutation {
    pub fn new(num_of_stairs: i32, max_step: i32) -> Self {
        Self {
            values: Vec::new(),
            idx: 0,
            num_of_stairs,
            max_step,
        }
    }
    pub fn is_valid(&self) -> bool {
        if self.values.iter().sum::<i32>() == self.num_of_stairs {
            true
        } else {
            false
        }
    }
    pub fn sum(&self) -> i32 {
        self.values.iter().sum()
    }
}
impl fmt::Display for Permutation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{:?} {}", self.idx, self.values, self.is_valid())
    }
}
