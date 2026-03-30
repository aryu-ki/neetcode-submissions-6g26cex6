impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = Vec::new();
        let operators = "+-*/";
        for token in &tokens {
            if operators.contains(token) {
                let (l, r) = match (stack.pop(), stack.pop()) {
                    (Some(l), Some(r)) => (r, l),
                    (_, _) => unreachable!()
                };
                stack.push(match token.as_str() {
                    "+" => l + r,
                    "-" => l - r,
                    "*" => l * r,
                    "/" => l / r, 
                    _ => unreachable!()
                });
            } else {
                stack.push(token.parse::<i32>().unwrap());
            }
        }
        stack.pop().unwrap()
    }
}
