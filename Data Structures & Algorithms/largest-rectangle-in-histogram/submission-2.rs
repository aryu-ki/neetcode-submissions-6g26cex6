impl Solution {
    pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
        let forward = merge_lefts(heights.into_iter().map(|height| (1usize, height))); 
        let backward = merge_lefts(forward.into_iter().rev());
    
        let mut max = 0i32;
        let mut min_height = i32::MAX;
        let mut total_width = 0i32;
    
        for (width, height) in backward.into_iter() {
            min_height = min_height.min(height);
            total_width += width as i32;
            max = max.max(width as i32 * height).max(min_height*total_width);
        }
    
        max
    }
}

fn merge_lefts(bars: impl Iterator<Item = (usize, i32)>) -> Vec<(usize, i32)> {
    let mut stack = Vec::new();
    for curr in bars {
        let mut curr = curr;
        while let Some(top) = stack.last() {
            if vertical_larger(top, &curr) || top.1 < curr.1 {
                break;
            } else {
                let top = stack.pop().unwrap();
                curr = (top.0 + curr.0, curr.1);
            }
        }
        stack.push(curr);
    }
    stack
}

fn vertical_larger(bar1: &(usize, i32), bar2: &(usize, i32)) -> bool {
    bar1.0 * bar1.1 as usize > horizontal(bar1, bar2)
}

fn horizontal(bar1: &(usize, i32), bar2: &(usize, i32)) -> usize {
    (bar2.0 + bar1.0) * bar2.1 as usize
}