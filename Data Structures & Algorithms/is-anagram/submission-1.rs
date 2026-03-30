use itertools::Itertools;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        // .counts() is a direct method on the iterator
        s.chars().counts() == t.chars().counts()
    }
}