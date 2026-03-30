use regex::Regex;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
    let s = s.as_bytes();
    if s.is_empty() { return true; }

    let mut i = 0i32;
    let mut j = s.len() as i32 - 1; // Works now because we checked is_empty()

    while i < j {
        // No casting needed here if i and j are both usize
        while i < j && !s[i as usize].is_ascii_alphanumeric() { i += 1; }
        while i < j && !s[j as usize].is_ascii_alphanumeric() { j -= 1; }

        if s[i as usize].to_ascii_lowercase() != s[j as usize].to_ascii_lowercase() {
            return false;
        }
        
        i += 1;
        // This is safe because i < j was true, so j must be at least 1
        j -= 1;
    }
    true
}
}
