/*
Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.

An input string is valid if:
    Open brackets must be closed by the same type of brackets.
    Open brackets must be closed in the correct order.
    Every close bracket has a corresponding open bracket of the same type.

Example 1:
    Input: s = "()"
    Output: true
Example 2:
    Input: s = "()[]{}"
    Output: true
Example 3:
    Input: s = "(]"
    Output: false

*/
struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {

        use std::collections::HashMap;
        let mut vec = vec![];
        let map = HashMap::from([
            ("{","}"),
            ("[","]"),
            ("(",")"),
        ]);

        // base case
        if s.is_empty() { return true; }

        for i in s.chars() {
            // any of {,(,[ get pushed in to the stack
            if map.contains_key(&i) { vec.push(i); }

            // any of },),] get checked vs. the top of the stack for a valid pair
            if map.get(&i).is_some() { vec.pop(); }

            // base case & endpoint
            if vec.is_empty() { return true; }
        }
        // end
        false
    }
}

fn main() {
    assert_eq!(Solution::is_valid("()".to_string()), true);
    assert_eq!(Solution::is_valid("()[]{}".to_string()), true);
    assert_eq!(Solution::is_valid("(]".to_string()), false);
}
