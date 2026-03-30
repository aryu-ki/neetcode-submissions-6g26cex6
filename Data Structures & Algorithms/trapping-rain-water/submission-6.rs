impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut stack: Vec<usize> = Vec::new();
        
        let mut res = 0;

        for i in 0..height.len() {
            while let Some(&top) = stack.last() {
                if height[top] <= height[i] {
                    let mid = stack.pop().unwrap();
                    if let Some(&left) = stack.last() {
                        let w = (i - left) as i32 - 1;
                        res += w * (height[i].min(height[left]) - height[mid]);
                    }
                } else {
                    break;
                }
            }
            stack.push(i);
        }
        res
    }
}