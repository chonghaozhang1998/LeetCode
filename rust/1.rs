// 两数之和
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();

        let len = nums.len();

        for i in 0..len {
            if let Some(val) = map.get(&(target - nums[i])) {
                if i != *val {
                    return vec![i as i32, *val as i32];
                }
            }
            map.insert(nums[i], i);
        }
        vec![]
    }
}
