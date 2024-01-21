struct Solution;

impl Solution {
    pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
        let mut h = heights.clone();
        let mut ans = Vec::with_capacity(names.len());
        let mut map = std::collections::HashMap::with_capacity(names.len());

        for (i, &val) in heights.iter().enumerate() {
            map.insert(val, names.get(i).cloned().unwrap());
        }

        h.sort();
        h.reverse();

        for i in h {
            ans.push(map.get(&i).cloned().unwrap());
        }

        ans
    }
}
