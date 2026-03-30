impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let mut freq = HashMap::new();
        let mut max = 0;
        let (mut l, mut r) = (0, 0);
        let rem = k;
        let mut res = 0;
        while r < s.len() {
            let mut at_r = freq.entry(s[r]).or_insert(0);
            *at_r += 1;
            max = max.max(*at_r);
            let size = r - l + 1;
            if size as i32 - max > k {
                *freq.entry(s[l]).or_insert(1) -= 1;
                l += 1;
            } else {
                res = res.max(size);
            }
            r += 1
        }
        res as i32
    }
}
