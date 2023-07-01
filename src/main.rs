#![allow(dead_code)]

// 1: Two Sum
mod two_sum {
    use std::collections::HashMap;

    fn two_sum(v: Vec<i32>, target: i32) -> Option<(i32, i32)> {
        let mut pairs = HashMap::with_capacity(v.len());

        for n in v {
            let d = target - n;

            if pairs.get(&d).is_some() {
                return Some((n, d));
            }

            pairs.insert(n, d);
        }

        None
    }
}

fn main() {
    
}
