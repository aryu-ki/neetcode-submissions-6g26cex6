impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let s = s.as_bytes();
        let mut set = HashSet::new();
        let (mut l, mut r) = (0, 0);
        let mut max = 0;

        while r < s.len() {
            if set.contains(&s[r]) {
                loop {
                    set.remove(&s[l]);
                    l += 1;
                    if l >= r || s[l - 1] == s[r] {
                        break;
                    }
                }
            } else {
                max = max.max((r - l) as i32 + 1);
            }
            set.insert(s[r]);
            r += 1;
        }
        max
    }
}
