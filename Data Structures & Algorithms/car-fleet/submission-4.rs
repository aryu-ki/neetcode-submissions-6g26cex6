impl Solution {
    pub fn car_fleet(target: i32,  position: Vec<i32>,speed: Vec<i32>) -> i32 {
        let mut stack = Vec::new();
        let mut sorted = position.into_iter()
            .zip(speed)
            .collect::<Vec<(i32, i32)>>();
        sorted.sort_by(|(lp, _), (rp, _)| lp.partial_cmp(rp).unwrap()); 

        for &(pos, speed) in &sorted {
            let ttf = (target - pos) as f32 / speed as f32;
            while let Some(&prev) = stack.last() {
                if prev <= ttf {
                    stack.pop();
                } else {
                    break;
                }
            } 
            stack.push(ttf);
        }

        stack.len() as i32
    }
}
