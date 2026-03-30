use itertools::Itertools;
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        strs.into_iter()
            .map(|s| {
                let char_counts = s.chars().fold([0 as i32; 27], |mut acc, p| {
                    acc[p as usize - 'a' as usize] += 1;
                    acc
                });
                (char_counts, s)
            })
            .into_group_map()
            .into_values()
            .collect()
    }
}
