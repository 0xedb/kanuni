struct Solution;

impl Solution {
    pub fn are_occurrences_equal(s: String) -> bool {
        let mut map = std::collections::HashMap::with_capacity(s.len());

        for i in s.bytes() {
            let val = map.entry(i).or_insert(0);
            *val += 1;
        }

        let &count = map.get(&s.bytes().nth(0).unwrap()).unwrap();

        for &v in map.values() {
            if v != count {
                return false;
            }
        }

        true
    }
}
