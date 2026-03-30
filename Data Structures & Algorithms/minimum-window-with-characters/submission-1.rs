use crate::hash_map::Entry::*;
use std::ops::RangeInclusive;
impl Solution {
    pub fn min_window(text: String, search: String) -> String {
        let text = text.as_bytes();
        let search_counts = search.clone().as_bytes().iter().fold(HashMap::new(), |mut acc, &x| {
            *acc.entry(x).or_insert(0) += 1;
            acc 
        });
        let mut need = search_counts.clone();
        let mut excess = HashMap::new();
        let (mut l, mut r) = (0, 0);
        let mut min: Option<(usize, usize)> = None;
        while r < text.len() {
            let at_r = text[r]; 
            if let Some(_) = search_counts.get(&at_r) {
                if let Occupied(mut entry) = need.entry(at_r) {
                    *entry.get_mut() -= 1;
                    if *entry.get() == 0 {
                        entry.remove();
                    }
                } else {
                    *excess.entry(at_r).or_insert(0) += 1;
                }
            }

            while need.is_empty() {
                if let None = min {
                    min = Some((l, r));
                } else {
                    if Self::size(min) > Self::size(Some((l, r))) {
                        min = Some((l, r));
                    }
                }
                let at_l = text[l];
                if let Some(_) = search_counts.get(&at_l)  {
                    if let Occupied(mut removing) = excess.entry(at_l) {
                        *removing.get_mut() -= 1;
                        if *removing.get() == 0 {
                            removing.remove();
                        }
                    } else {
                        *need.entry(at_l).or_insert(0) += 1;
                    }
                }
                l += 1;
            }
            r += 1;
        }

        if let Some(min) = min {
            Self::substring(text, min.0..=min.1)
        } else {
            "".to_string()
        }
    }

    fn substring(text: &[u8], r: RangeInclusive<usize>) -> String {
        str::from_utf8(&text[r]).unwrap().to_string()
    }

    fn size(indexes: Option<(usize, usize)>) -> usize {
        match indexes {
            Some((l, r)) => r - l + 1,
            None => usize::MAX
        } 
    }
}
