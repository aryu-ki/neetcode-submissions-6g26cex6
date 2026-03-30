impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut m = HashMap::new();
        for (i, x) in nums.into_iter().enumerate() {
            let diff = target - x;
            if let Some(&j) = m.get(&diff) {
                return vec![j as i32, i as i32]
            }

            m.insert(x, i);
        }

        vec![]
    }
}
