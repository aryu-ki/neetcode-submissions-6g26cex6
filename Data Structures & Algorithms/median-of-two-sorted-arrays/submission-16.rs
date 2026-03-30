impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        if nums1.len() > nums2.len() {
            return Self::find_median_sorted_arrays(nums2, nums1);
        }
        if nums2.is_empty() {
            let n = nums2.len();
            if n % 2 == 0 {
                return (nums2[n / 2] + nums2[n / 2 - 1]) as f64 / 2f64;
            } else {
                return nums2[n/2] as f64;
            }
        }
        let size = (nums2.len() + nums1.len() + 1) as i32 / 2;
        if let (Some(l), Some(r)) = (nums1.last(), nums2.first()) {
            if l <= r {
                if nums1.len() == nums2.len() {
                    return (r + l) as f64 / 2f64;
                } 
                let m = size as usize - nums1.len();
                if (nums1.len() + nums2.len()) % 2 == 0 {
                    return (nums2[m] + nums2[m+1]) as f64 / 2f64;
                } else {
                    return nums2[m] as f64;
                }
            }
        };
        let (mut l, mut r) = (0i32, nums1.len() as i32);
        while l <= r {
            let left_m = l + (r - l) / 2;
            let right_m = size - left_m;
            let (mut l1, mut l2, mut r1, mut r2) = (i32::MIN, i32::MAX, i32::MIN, i32::MAX);
            if left_m > 0 {l1 = nums1[left_m as usize -1];}
            if left_m < nums1.len() as i32 {l2 = nums1[left_m as usize];}
            if right_m > 0 {r1 = nums2[right_m as usize -1];} 
            if right_m < nums2.len() as i32 {r2 = nums2[right_m as usize];} 
            if l1 <= r2 && r1 <= l2 {
                if (nums1.len() + nums2.len()) % 2 == 0 {
                    return (l1.max(r1) as f64 + r2.min(l2) as f64) / 2f64;
                } else {
                    return l1.max(r1) as f64;
                }
            } else if l1 > r2 {
                r = left_m - 1;
            } else {
                l = left_m + 1;
            }
        }
        -1f64
    }
}
