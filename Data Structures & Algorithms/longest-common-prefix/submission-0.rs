impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.len() == 0 {
            return String::from("");
        }
        if (strs.len() == 1) {
            return String::from(strs[0].clone());
        }
        let mut result = String::new();
        for i in 0.. {
            let mut curr_char: Option<char> = None;
            for s in strs.iter() {
                if i >= s.len() {
                    return result;
                }
                match curr_char {
                    Some(c) if c != s.chars().nth(i).expect("dumb") => {return result;}, 
                    None => {curr_char = s.chars().nth(i);},
                    _ => ()
                }
            }
            result.push(curr_char.expect("something went wrong"));
            curr_char = None;
        }
        result
    }
}
