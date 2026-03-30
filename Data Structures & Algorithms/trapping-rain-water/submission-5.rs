impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut left_highest = vec![0; height.len()];
        let mut right_highest = vec![0; height.len()];
        let mut max = height[0];
        for i in 1..height.len() {
            max = max.max(height[i-1]);
            left_highest[i] = max;
        }
        max = height[height.len() - 1];
        for i in (0..height.len()-1).rev() {
            max = max.max(height[i+1]);
            right_highest[i] = max;
        }

        let mut res = 0;
        for i in 1..(height.len() - 1) {
            res += 0.max(left_highest[i].min(right_highest[i]) - height[i]);
        }
        res
    }
}
