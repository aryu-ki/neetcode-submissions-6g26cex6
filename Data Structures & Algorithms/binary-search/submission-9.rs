impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut l, mut r) = (0usize, nums.len() as usize - 1);
        while l <= r {
            let mid = l + (r - l) / 2;
            if let Some(m) = nums.get(mid) {
                if *m < target {
                    l = mid + 1;
                } else if *m > target {
                    r = mid - 1;
                } else {
                    return mid as i32;
                }
            } else {
                break;
            }
        }

        -1
    }
}
