impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        let l = nums.len();
        let mut j = nums.len() - 1;
        let mut i = 0;
        while i <= j && j < l {
            if nums[i] == val {
                let tmp = nums[i];
                nums[i] = nums[j];
                nums[j] = tmp;
                j -= 1;
                i -= 1;
            }
            i += 1;
        }
        (j + 1) as i32
    }
}
