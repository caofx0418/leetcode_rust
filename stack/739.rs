impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut res = vec![0; temperatures.len()];
        let mut stack = Vec::new();

        for i in 0..temperatures.len() {
            while !stack.is_empty() && temperatures[stack[stack.len() - 1]] < temperatures[i] {
                let out_pos = stack.pop().unwrap();
                res[out_pos] = (i - out_pos) as i32;
            }
            stack.push(i);
        }

        res
    }
}