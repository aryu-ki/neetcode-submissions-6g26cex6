impl Solution {
    pub fn max_area(heights: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0, heights.len() - 1);
        let mut res = -1;
        while l < r {
            res = res.max(heights[l].min(heights[r]) * (r as i32 - l as i32));
            if heights[l] > heights[r] {
                r -= 1;
            } else {
                l += 1;
            }
        }
        res
    }
}
