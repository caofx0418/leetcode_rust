use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();

        for str in strs.iter() {
            let mut chars: Vec<char> = vec![];
            for c in str.chars() {
                chars.push(c);
            }
            chars.sort();
            let k: String = chars.into_iter().collect();
            let mut res = map.get(&k);
            if res != None {
                let mut v = res.unwrap().to_vec();
                v.push(str.clone());
                map.insert(k.clone(), v);
            } else {
                let v = vec![str.clone()];
                map.insert(k.clone(), v);
            }
        }

        let mut result: Vec<Vec<String>> = Vec::new();
        for v in map.values() {
            result.push(v.to_vec());
        }

        return result;
    }
}