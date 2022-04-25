impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut vec = Vec::new();
        for token in tokens.iter() {
            if let Ok(v) = token.parse::<i32>() {
                vec.push(v);
                continue;
            }

            let right = vec.pop().unwrap();
            let left = vec.pop().unwrap();

            match token as &str {
                "+" => vec.push(left + right),
                "-" => vec.push(left - right),
                "*" => vec.push(left * right),
                "/" => vec.push(left / right),
                _ => unreachable!(),
            }
        }
        vec.pop().unwrap()
    }
}