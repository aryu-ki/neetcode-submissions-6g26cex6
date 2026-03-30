use itertools::Itertools;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut counts = HashMap::new();
        let mut freq = vec![vec![]; nums.len() + 1];
        for &num in &nums {
            *counts.entry(num).or_insert(0usize) += 1;
        }
        for (&k, &v) in &counts {
            freq[v].push(k);
        }

        let mut res = Vec::new();
        for i in (1..freq.len()).rev() {
            for &el in &freq[i] {
                res.push(el);
                if res.len() == k {
                    return res;
                }
            }
        }
        res
    }
}
