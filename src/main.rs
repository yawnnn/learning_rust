#![allow(dead_code)]
//#![feature(iter_collect_into)]

// 1: Two Sum
mod two_sum {
    use std::collections::HashMap;

    pub fn two_sum(v: &[i32], target: i32) -> Option<(i32, i32)> {
        let mut pairs = HashMap::with_capacity(v.len());

        for &n in v {
            let d = target - n;

            if pairs.get(&d).is_some() {
                return Some((n, d));
            }

            pairs.insert(n, d);
        }

        None
    }
}

// 2: Roman numerals to integer
mod roman_numerals {
    use RomanNumeral::*;

    #[derive(PartialEq, Eq, Hash)]
    enum RomanNumeral {
        I, V, X, L, C, D, M,
    }

    impl TryFrom<char> for RomanNumeral {
        type Error = ();

        fn try_from(c: char) -> Result<Self, Self::Error> {
            match c {
                'I' | 'i' => Ok(I), 'V' | 'v' => Ok(V),
                'X' | 'x' => Ok(X), 'L' | 'l' => Ok(L),
                'C' | 'c' => Ok(C), 'D' | 'd' => Ok(D),
                'M' | 'm' => Ok(M),
                _ => Err(()),
            }
        }
    }

    impl From<RomanNumeral> for i32 {
        fn from(rn: RomanNumeral) -> i32 {
            match rn {
                I => 1, V => 5,
                X => 10, L => 50,
                C => 100, D => 500,
                M => 1000,
            }
        }
    }

    pub fn roman_to_integer(s: &str) -> i32 {
        let numerals: Vec<i32> = s
            .chars()
            .filter_map(|c| RomanNumeral::try_from(c).ok())
            .map(|numeral| numeral.into())
            .collect();

        let mut total = 0;

        for (n1, n2) in numerals.iter().zip(numerals.iter().skip(1)) {
            if n2 > n1 {
                total -= n1;
            } else {
                total += n1;
            }
        }

        if let Some(&n) = numerals.last() {
            total += n;
        }

        total
    }
}

// 3: Is palindrome
mod is_palindrome {
    pub fn is_palindrome_1(mut n: i32) -> bool {
        let mut half = 0;
        
        if n < 0 || (n != 0 && n % 10 == 0) { 
            return false; 
        }

        while n > half {
            half = (half * 10) + (n % 10);
            n /= 10;
        }

        n == half || n == (half / 10)
    }

    pub fn is_palindrome_2(n: i32) -> bool {
        let s = n.to_string();
        let len = s.len() as f32;
        let lower_half = (len / 2.).ceil() as usize;
        let upper_half = (len / 2.).ceil() as usize;
        
        s.chars().take(lower_half).eq(s.chars().rev().take(upper_half))
    }
}

mod longest_common_prefix {
    pub fn longest_common_prefix<T: AsRef<str>>(v: &[T]) -> Option<String> {
        let mut prefix = v.first()?.as_ref().to_owned();

        for s in v {
            while !s.as_ref().starts_with(&prefix) {
                prefix.pop();
            }

            if prefix.is_empty() {
                break;
            }
        }

        return (!prefix.is_empty()).then_some(prefix);
    }

    pub fn longest_common_prefix_fold<T: AsRef<str>>(v: &[T]) -> Option<String> {
        let prefix = v.iter()
            .map(|s| s.as_ref())
            .fold(v.first()?.as_ref().to_owned(), |acc, curr| acc
                .chars()
                .zip(curr.chars())
                .take_while(|(a, b)| a == b)
                .map(|(c,_)| c)
                .collect());

        return (!prefix.is_empty()).then_some(prefix);
    }

    pub fn longest_common_prefix_reduce_copy<T: AsRef<str>>(v: &[T]) -> Option<String> {
        let v: Vec<String> = v.into_iter().map(|s| s.as_ref().to_owned()).collect();

        v.into_iter().reduce(|accumulator, current| {
            accumulator.chars()
                .zip(current.chars())
                .take_while(|(a, b)| a == b)
                .map(|(c,_)| c)
                .collect()
        })
        .and_then(|s| (!s.is_empty()).then_some(s))
    }

    // Nightly feature: collect_into()
    // pub fn longest_common_prefix_reduce<T: AsRef<str>>(v: &[T]) -> Option<String> {
    //     let mut s = String::with_capacity(
    //         v.first()
    //         .and_then(|s| Some(s.as_ref().len()))
    //         .unwrap_or(0)
    //     );

    //     v.iter().map(|s| s.as_ref()).reduce(|accumulator, current| {
    //         accumulator.chars()
    //             .zip(current.chars())
    //             .take_while(|(a, b)| a == b)
    //             .map(|(c,_)| c)
    //             .collect_into(&mut s)
    //             .as_str()
    //     })
    //     .and_then(|s| (!s.is_empty()).then_some(s.to_owned()))
    // }
}

mod valid_parenthesis {
    pub fn valid_parenthesis(s: &str) -> bool {
        let mut openings = String::with_capacity((s.len() as f64 / 2.).ceil() as usize);

        for c in s.chars() {
            if "([{".contains(c) {
                openings.push(c);
            } else if let Some(opening) = openings.pop() {
                if opening != 
                    match c {
                        ')' => '(', ']' => '[', '}' => '{',
                        _   => '\0', } 
                {
                    return false;
                }
            }
        }

        openings.is_empty()
    }
}

mod merge_sorted_list {
    use std::collections::LinkedList;

    pub fn merge_sorted_list(l1: LinkedList<i32>, l2: LinkedList<i32>) -> LinkedList<i32> {
        let mut result: LinkedList<i32> = LinkedList::new(); 
        let mut i1 = l1.iter().peekable();
        let mut i2 = l2.iter().peekable();

        while let (Some(&&x), Some(&&y)) = (i1.peek(), i2.peek()) {
            if x <= y {
                result.push_back(x);
                i1.next();
            } else {
                result.push_back(y);
                i2.next();
            }
        }

        result.extend(i1);
        result.extend(i2);

        result
    }
}

fn main() {
    let _v = vec![String::from("flow"), String::from("aflower"), String::from("flowl")];
    let _v2 = ["aflow", "flower", "flowl"];
    let _v3: Vec<String> = Vec::new(); 

    println!("{:?}", longest_common_prefix::longest_common_prefix_fold(&_v2));
}
