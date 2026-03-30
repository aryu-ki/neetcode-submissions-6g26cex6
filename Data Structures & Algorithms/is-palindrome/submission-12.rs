use regex::Regex;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
    let s = s.as_bytes();
    if s.is_empty() { return true; }

    let mut i = 0;
    let mut j = s.len() - 1;

    while i < j {
        // use usize directly, but ensure j > 0 before decrementing
        while i < j && !s[i].is_ascii_alphanumeric() { i += 1; }
        while i < j && !s[j].is_ascii_alphanumeric() { j -= 1; }

        if i < j && s[i].to_ascii_lowercase() != s[j].to_ascii_lowercase() {
            return false;
        }
        
        i += 1;
        if j > 0 { j -= 1; } else { break; } // Safe decrement for usize
    }
    true
}
}
