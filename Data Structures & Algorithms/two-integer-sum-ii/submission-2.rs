impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut l, mut r) = (0i32, numbers.len() as i32 - 1);
        while l < r {
            let sum = numbers[l as usize] + numbers[r as usize]; 
            if sum == target {
                return vec![l + 1, r + 1];
            }
            if sum < target {
                l += 1;
            } else {
                r -= 1;
            }
        }
        unreachable!();
    }
}
