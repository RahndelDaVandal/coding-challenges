// This problem was asked by Uber.

// Given an array of integers, return a new array such that
// each element at index i of the new array is the product 
// of all the numbers in the original array except the one at i.

// For example, if our input was [1, 2, 3, 4, 5], 
// the expected output would be [120, 60, 40, 30, 24]. 
// If our input was [3, 2, 1], 
// the expected output would be [2, 3, 6].

// Follow-up: what if you can't use division?

use std::collections::VecDeque;

fn main() {
    let nums1: Vec<i32> = vec!(1, 2, 3, 4, 5);
    let nums2: Vec<i32> = vec!(3, 2, 1);

    print!("{nums1:?} => ");
    println!("{:?}", solution(nums1));
    print!("{nums2:?} => ");
    println!("{:?}", solution(nums2));
}

fn solution(input: Vec<i32>) -> Vec<i32> {
    let mut nums = VecDeque::from(input);
    let mut output: Vec<i32> = Vec::new();
    for _ in 0..nums.len() {
        let n = nums.pop_front().unwrap();
        output.push(nums.iter().product());
        nums.push_back(n);
    }
    output
}

#[cfg(test)]
pub mod test {
    use super::solution;

    #[test]
    fn verify_solution() {
        let case1: Vec<i32> = vec!(1, 2, 3, 4, 5);
        let case1_expected: Vec<i32> = vec!(120, 60, 40, 30, 24);
        let case2: Vec<i32> = vec!(3, 2, 1);
        let case2_expected: Vec<i32> = vec!(2, 3, 6);

        assert_eq!(case1_expected, solution(case1));
        assert_eq!(case2_expected, solution(case2));
    }
}
