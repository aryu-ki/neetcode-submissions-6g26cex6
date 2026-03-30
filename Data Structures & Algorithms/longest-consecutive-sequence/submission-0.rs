impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut mp: HashMap<i32, i32> = HashMap::new();
        let mut res = 0;
        for &num in &nums {
            if let None = mp.get(&num) {
                let left = *mp.get(&(num - 1)).unwrap_or(&0);
                let right = *mp.get(&(num + 1)).unwrap_or(&0);
                let new_len = left + right + 1;
                mp.insert(num, new_len);
                mp.insert(num - left, new_len);
                mp.insert(num + right, new_len);
                res = res.max(new_len);
            }
        }
        res
    }
}
