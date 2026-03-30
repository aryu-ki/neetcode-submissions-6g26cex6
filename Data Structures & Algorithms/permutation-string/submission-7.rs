impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        if s1.len() > s2.len() {
            return false;
        }
        let source = s2.as_bytes();
        let search = s1;
        let initial: HashMap<u8, i32> = search.as_bytes().into_iter()
            .fold(HashMap::new(), |mut acc, &x| {
                *acc.entry(x).or_insert(0) += 1 as i32;
                acc
            });

        let (mut l, mut r) = (0, search.len() - 1);
        let mut curr = initial.clone();
        for i in 0..=r {
            let ith = curr.get_mut(&source[i]);
            if let Some(s) = ith {
                *s -= 1; 
                if *s == 0 {
                    curr.remove(&source[i]); 
                }
            } else {
                curr.insert(source[i], -1);
            }
        }
        while r < source.len() {
            if curr.is_empty() {
                return true;
            }

            let mut at_l = curr.entry(source[l]).or_insert(0);
            *at_l += 1;
            if *at_l == 0 {
                curr.remove(&source[l]);
            } 

            l += 1;
            r += 1; 

            if r >= source.len() {
                return false;
            }
            let mut at_r = curr.entry(source[r]).or_insert(0);
            *at_r -= 1;
            if *at_r == 0 {
                curr.remove(&source[r]);
            }
        }
        false
    }
}
