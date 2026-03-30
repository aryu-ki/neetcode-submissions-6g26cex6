impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let (mut l, mut r) = (0i32, *piles.iter().max().unwrap());
        let mut res = -1;
        while l <= r {
            let mut curr = 0f64;
            let i = l + (r - l) / 2;
            for &pile in &piles {
                curr += (pile as f64 / i as f64).ceil();
            }
            if curr <= h as f64 {
                r = i - 1;
                res = i;
            } else {
                l = i + 1;
            }
        }
        res
    }
}
