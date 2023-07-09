#![allow(dead_code)]

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

fn main() {
    println!("{:?}", two_sum::two_sum(&[2,7,11,15], 9));
    println!("{:?}", two_sum::two_sum(&[3,2,4], 6));
    println!("{:?}", two_sum::two_sum(&[3,3], 6));
}
