impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let w = matrix[0].len() as i32;
        let (mut l, mut r) = (0i32, (matrix.len() * matrix[0].len() - 1) as i32);
        while l <= r {
            let i = l + (r - l) / 2;
            let el = matrix[(i/w) as usize][(i%w) as usize];
            if el < target {
                l = i + 1;
            } else if el > target {
                r = i - 1;
            } else {
                return true;
            }
        }
        false
    }
}
