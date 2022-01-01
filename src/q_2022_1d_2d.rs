use std::collections::HashMap;
use std::vec::Vec;

struct Solution;

impl Solution {
    pub fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
        if m * n != original.len() as i32 {
            return vec![];
        }

        let mut out = vec![];
        let mut index = 0;
        for _ in 0..m {
            let mut one_line = vec![];
            for _ in 0..n {
                one_line.push(original[index]);
                index += 1;
            }
            out.push(one_line);
        }

        return out;
    }
}

#[test]
fn test() {
    // 输入：
    let original = vec![1, 2, 3, 4];
    let m = 2;
    let n = 2;
    let out = vec![vec![1, 2], vec![3, 4]];
    assert_eq!(Solution::construct2_d_array(original, m, n), out);

    let original = vec![1, 2];
    let m = 1;
    let n = 1;
    let out: Vec<Vec<i32>> = vec![];
    assert_eq!(Solution::construct2_d_array(original, m, n), out);
}
