impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0i32, height.len() as i32 - 1);
        let mut res = 0;
        let (mut left_highest, mut right_highest) = (0, 0);
        while l < r {
            if height[l as usize] < height[r as usize] {
                left_highest = left_highest.max(height[l as usize]);
                l += 1;
                res += 0.max(left_highest - height[l as usize]);
            } else {
                right_highest = right_highest.max(height[r as usize]);
                r -= 1;
                res += 0.max(right_highest - height[r as usize]);
            }
        }
        res
    }
}