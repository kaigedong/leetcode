use std::collections::HashMap;
use std::vec::Vec;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut a_map = HashMap::new();

        for (index, &value) in nums.iter().enumerate() {
            if a_map.contains_key(&(target - value)) {
                return vec![a_map[&(target - value)] as i32, index as i32];
            } else {
                a_map.insert(value , index);
            }
        }

        vec![0, 1]
    }
}

#[test]
fn two_sum() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let solution = vec![0, 1];
    assert_eq!(Solution::two_sum(nums, target), solution);

    let nums = vec![3, 2, 4];
    let target = 6;
    let solution = vec![1, 2];
    assert_eq!(Solution::two_sum(nums, target), solution);

    let nums = vec![3, 3];
    let target = 6;
    let solution = vec![0, 1];
    assert_eq!(Solution::two_sum(nums, target), solution);
}
