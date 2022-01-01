struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        if x < 10 {
            return true;
        }

        let s = x.to_string();
        if s.len() % 2 == 1 {
            s[0..(s.len() / 2)].to_string()
                == s[(s.len() / 2 + 1)..].chars().rev().collect::<String>()
        } else {
            s[0..(s.len() / 2)].to_string() == s[(s.len() / 2)..].chars().rev().collect::<String>()
        }
    }
}

#[test]
fn test_palindrome() {
    assert_eq!(Solution::is_palindrome(121), true);
    assert_eq!(Solution::is_palindrome(-121), false);
    assert_eq!(Solution::is_palindrome(1), true);
    assert_eq!(Solution::is_palindrome(-1), false);
    assert_eq!(Solution::is_palindrome(10), false);
    assert_eq!(Solution::is_palindrome(-101), false);
    assert_eq!(Solution::is_palindrome(11), true);
    assert_eq!(Solution::is_palindrome(1001), true);
}
