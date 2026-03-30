impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut mp: HashMap<i32, i32> = HashMap::new();
        let mut res = 0;
        for &num in &nums {
            if let None = mp.get(&num) {
                let left = mp.get(&(num - 1));
                let right = mp.get(&(num + 1));
                let new_len = match (left, right) {
                    (Some(l), Some(r)) => l + r + 1,
                    (l, r) => l.or(r).map_or(1, |len| len + 1)
                };
                let right = *mp.get(&(num + 1)).unwrap_or(&0);
                let left = *mp.get(&(num - 1)).unwrap_or(&0);
                mp.insert(num, new_len);
                mp.insert(num + right, new_len);
                mp.insert(num - left, new_len);
                res = res.max(new_len);
            }
        }
        res
    }
}
