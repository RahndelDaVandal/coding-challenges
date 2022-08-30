#![allow(dead_code, unused_variables)]
// This problem was recently asked by Google
// Given a list of numbers and number `k`. Return whether 
// any two numbers from the list add up to `k`.
// [10, 15, 3, 7] k=17 => true (10+7=17)
// BONUS: Can you do it in one pass?

fn main() {
    let nums:Vec<i32> = vec!(10, 15, 3, 7);
    let k: i32 = 17;

    println!("{}", solution(nums, k));
}

pub fn solution(mut nums: Vec<i32>, k: i32) -> bool {
    let n = nums.pop().unwrap();
    let check:Vec<&i32> = nums.iter().filter(|x| *x + n == k).collect();
    if check.len() > 0 {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn should_return_true() {
        let nums:Vec<i32> = vec!(10,15,3,7);
        let k: i32 = 17;
        let result = solution(nums, k);
        assert_eq!(true, result);
    }
    #[test]
    fn should_return_false() {
        let nums:Vec<i32> = vec!(10,15,3,6);
        let k: i32 = 17;
        let result = solution(nums, k);
        assert_eq!(false, result);
    }
}
