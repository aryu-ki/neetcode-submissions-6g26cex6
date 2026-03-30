impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut stack: Vec<usize> = Vec::new();
        let mut res = vec![0i32; temperatures.len()];
        for i in 0..temperatures.len() {
            while let Some(&t) = stack.last() {
                if temperatures[t] < temperatures[i] {
                    res[t] = (i - t) as i32;
                    stack.pop();
                } else {
                    break;
                }
            }
            stack.push(i);
        }
        res
    }
}
