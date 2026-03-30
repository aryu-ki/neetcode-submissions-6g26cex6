impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let s = s.as_bytes();
        let mut res = 0;

        for i in 0..s.len() {
            let mut char_set = HashSet::new();
            for j in i..s.len() {
                if char_set.contains(&s[j]) {
                    break;
                }
                char_set.insert(s[j]);
            }
            res = res.max(char_set.len());
        }
        res as i32
    }
}