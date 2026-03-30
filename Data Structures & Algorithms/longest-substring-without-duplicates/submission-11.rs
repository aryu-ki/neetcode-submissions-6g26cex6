impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.len() <= 1 {
            return s.len() as i32;
        }
        let mut unique = HashSet::new();
        let (mut l, mut r) = (0, 0);
        let mut res = 0;
        while r < s.len() {
            let at_r = s.chars().nth(r).unwrap();
            if unique.contains(&at_r) {
                while l < r {
                    let at_l = s.chars().nth(l).unwrap();
                    l += 1;
                    unique.remove(&at_l);
                    if at_l == at_r {
                        break;
                    }
                }
            }
            unique.insert(at_r);
            res = res.max(unique.len());
            r += 1;
        }
        res as i32
    }
}
