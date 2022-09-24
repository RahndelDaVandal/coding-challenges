// This problem was asked by Stripe.

// Given an array of integers, find the first missing positive 
// integer in linear time and constant space. In other words, 
// find the lowest positive integer that does not exist in the array. 
// The array can contain duplicates and negative numbers as well.

// For example, the input [3, 4, -1, 1] should give 2. The input [1, 2, 0] should give 3.

fn main() {
    let nums: Vec<i32> = vec!(3, 4, -1, 1);
    println!("{}", solution(nums));
}

fn solution(mut nums: Vec<i32>) -> i32 {
    nums.sort();
    let range = 1..=nums.len() as i32;
    let test: Vec<i32> = range
        .into_iter()
        .filter(|x| !nums.contains(x))
        .collect();
    println!("{test:?}");
    test[0]
}

#[cfg(test)]
pub mod test{
    use super::solution;

    #[test]
    fn test_given_test_case() {
        let nums: Vec<i32> = vec!(3, 4, -1, 1);
        assert_eq!(2, solution(nums));
    } 
}
