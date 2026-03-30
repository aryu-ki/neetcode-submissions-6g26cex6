impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums.clone();
        nums.sort_unstable();
        let mut res = vec![];
        for (i, &num) in nums.iter().enumerate() {
            if num > 0 { break; }
            if i > 0 && num == nums[i - 1] {
                continue;
            }
            let mut l = i + 1;
            let mut r = nums.len() - 1;
            while l < r {
                let sum = nums[l] + nums[r] + num;
                if sum == 0 {
                    res.push(vec![nums[i], nums[l], nums[r]]);
                    l += 1;
                    r -= 1;
                    while l < r && nums[l] == nums[l - 1] {
                        l += 1;
                    }
                } else if sum < 0 {
                    l += 1;
                } else if sum > 0 {
                    r -= 1;
                }

            }
        }
        res
    }
}
