use std::time::SystemTime;
struct LRUCache {
    m: HashMap<i32, (i32, i32)>,
    cap: i32,
    seq: i32
}

impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        LRUCache{m: HashMap::with_capacity(capacity as usize), cap: capacity, seq: 0}
    }

    pub fn get(&mut self, key: i32) -> i32 {
        let entry = self.m.get_mut(&key);
        match entry {
            Some(entry) => {
                entry.1 = self.seq;
                self.seq += 1;
                entry.0
            }, 
            None => {
                -1i32
            }
        }
    }

    pub fn put(&mut self, mut key: i32, value: i32) {
        if self.m.contains_key(&key) {
            self.m.remove(&key);
        } else if self.m.len() == self.cap as usize {
            let mut min: Option<(i32, i32)> = None;
            for (k, v) in &self.m{
                if min.is_none() || min.as_ref().unwrap().1 > v.1 {
                    min = Some((*k, v.1));
                }
            } 
            self.m.remove(&min.as_ref().unwrap().0);
        }
        let entry = self.m.insert(key,(value, self.seq));
        self.seq += 1;
    }
}
