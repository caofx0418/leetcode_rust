impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        let mut count = [0; 26];
        let mut used = [false; 26];
        let mut vec = Vec::new();
        let to_index = |x: u8| -> usize { (x - b'a') as usize };

        for &e in s.as_bytes() {
            count[to_index(e)] += 1;
        }

        for &e in s.as_bytes() {
            let index = to_index(e);
            count[index] -= 1;
            if used[index] {
                continue;
            }

            while let Some(&u) = vec.last() {
                let out_index = to_index(u as u8);
                if u as u8 > e && count[out_index] > 0 {
                    vec.pop();
                    used[out_index] = false;
                } else {
                    break;
                }
            }

            used[index] = true;
            vec.push(e as char);
        }

        vec.iter().collect()
    }
}