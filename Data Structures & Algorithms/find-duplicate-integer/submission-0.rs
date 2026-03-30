impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut set = HashSet::with_capacity(nums.len());

        for num in &nums {
            if !set.insert(*num) {
                return *num; 
            }
        }

        unreachable!()
    }
}
