impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() <= 1 { 
            return 0;
        }
        let (mut l, mut r) = (0usize, 1usize);
        let mut res = 0i32;
        while r < prices.len() {
            if prices[l] > prices[r] {
                l = r;
            } else {
                res = res.max(prices[r] - prices[l]);
            }
            r += 1;
        }

        res 
    }
}
