impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut counts = HashMap::new(); 
        for v in nums.iter() {
            *counts.entry(v).or_insert(0) += 1;
        }
        let max = counts.iter()
            .max_by_key(|&(&key, &val)| val);

        if let Some((&k, v)) = max {
            return *k;
        }
        
        unreachable!();
    }
}
