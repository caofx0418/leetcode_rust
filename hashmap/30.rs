use std::collections::HashMap;

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        if s.len() < words.len() * words[0].len() {
            return vec![];
        }

        let mut hash: HashMap<&str, i32> = HashMap::new();
        for word in words.iter() {
            *hash.entry(word).or_insert(0) += 1;
        }

        // println!("{:?}", hash);
        let mut res:Vec<i32> = vec![];

        let n = words.len();
        let word_len = words[0].len();
        let mut found: HashMap<&str, i32> = HashMap::new();
        for i in 0..=(s.len() - n * word_len) {
            found.clear();
            let num = 0;
            for index in 0..n {
                let start = i + index * word_len;
                let word = &s[start..start + word_len];

                if hash.contains_key(word) {
                    *found.entry(word).or_insert(0) += 1;
                    if found.get(word) > hash.get(word) {
                        break;
                    }
                } else {
                    break;
                }
            }

            if found.eq(&hash) {
                res.push(i as i32);
            }
        }

        return res;
    }
}