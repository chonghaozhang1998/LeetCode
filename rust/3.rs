// 无重复字符的最长子串
use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut map = HashMap::new();
        let mut ans = 0;
        let mut index = 0;
        let mut count = 0;
        let mut left = 0;
        for c in s.chars() {
            if let Some(val) = map.get(&(c)) {

            } else {

            }
            map.insert(*c, index);
            index += 1;
        }
        ans as i32
    }
}
