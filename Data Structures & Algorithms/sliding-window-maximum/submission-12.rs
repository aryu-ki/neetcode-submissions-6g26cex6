impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut queue = BinaryHeap::with_capacity(nums.len());
        let mut result = vec![0; nums.len() - k as usize + 1];

        let (mut l, mut r) = (0usize, (k - 1) as usize);
        for i in l..=r {
            queue.push((nums[i], i));
        }

        while r < nums.len() {
            result[l] = queue.peek().unwrap().0;
            r += 1;
            if r >= nums.len() {
                break;
            }
            queue.push((nums[r], r));
            while queue.peek().unwrap().1 + k as usize <= r {
                queue.pop();
            } 
            l += 1;
        }

        result
    }
}
