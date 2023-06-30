/*
Given an integer x, return true if x is a palindrome, and false otherwise.

Example 1:
Input: x = 121
Output: true
Explanation: 121 reads as 121 from left to right and from right to left.

Example 2:
Input: x = -121
Output: false
Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.

Example 3:
Input: x = 10
Output: false
Explanation: Reads 01 from right to left. Therefore it is not a palindrome.

Constraints:
    -231 <= x <= 231 - 1
*/

struct Solution; 

impl Solution {
    /*
    pub fn palindrome(x: i32) -> bool {

        // base case: no negatives allowed
        if &x < 0 {
            return false;
        }

        use std::vec::Vec;
        
        let mut stack: i32 = vec![];
        let s = &x.to_string();
        let n = (&s.len());

        for i in 0..n {
            while stack.isnotempty() {
                //
                // stack first half of the number
                if i < n/2 {
                    stack.pop(s[i]);
                }

                // ignore middle integer (only occurs when length is an odd number) 
                if i == n/2 {}

                // check the second half versus the first
                if i > n/2 {
                    if stack.last() == s[i] {
                        stack.pop();
                    }
                }
                r
            }
            return true;
        }
        false
    }*/
    pub fn palindrome(x: i32) -> bool {
        let y = &x.to_string();
        let z: &String = &y.chars().rev().collect();
        if &y == &z {return true};
        false
    } 
}

fn main() {
    assert_eq!(Solution::palindrome(121), true);
    assert_eq!(Solution::palindrome(-121), false);
    assert_eq!(Solution::palindrome(10), false);
}
