impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut queue = BinaryHeap::with_capacity(nums.len());
        let mut result = Vec::new();
        for i in 0..nums.len() {
            queue.push((nums[i], i));
            if i >= k - 1 {
                while queue.peek().unwrap().1 + k <= i {
                    queue.pop();
                } 
                result.push(queue.peek().unwrap().0);
            }
        }

        result
    }
}
