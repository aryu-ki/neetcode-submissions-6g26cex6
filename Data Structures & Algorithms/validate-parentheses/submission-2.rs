impl Solution {
    pub fn is_valid(s: String) -> bool {
        let s = s.as_bytes();
        let mut stack: Vec<u8> = Vec::new();
        for i in 0..s.len() {
            let expected = match s[i] as char {
                ']' => Some('['),
                '}' => Some('{'),
                ')' => Some('('),
                _ => None
            };
            if let Some(expected) = expected {
                if let Some(actual) = stack.pop() {
                    if expected != actual as char {
                        return false;
                    }
                } else {
                    return false;
                }
            } else {
                stack.push(s[i]);
            }
        }
        stack.is_empty()
    }
}
