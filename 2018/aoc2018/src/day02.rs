use std::collections::HashMap;

use lines;

fn part1(lines: &Vec<String>) -> i64 {
    let mut twocount = 0;
    let mut threecount = 0;
    for l in lines.iter() {
        let mut hm = HashMap::new();
        for c in l.chars() {
            *hm.entry(c).or_insert(0) += 1;
        }
        let mut has_two = false;
        let mut has_three = false;
        for v in hm.values() {
            match v {
                2 => has_two = true,
                3 => has_three = true,
                _ => (),
            }
        }
        if has_two {
            twocount += 1;
        }
        if has_three {
            threecount += 1;
        }
    }

    twocount * threecount
}

fn single_diff(l1: &String, l2: &String) -> i64 {
    let mut diff = -1;

    let mut pos = 0;
    for (c1, c2) in l1.chars().zip(l2.chars()) {
        if c1 != c2 {
            if diff >= 0 {
                return -1;
            } else {
                diff = pos;
            }
        }
        pos += 1;
    }
    diff
}

fn part2(lines: &Vec<String>) -> String {
    for l1 in 0..(lines.len() - 1) {
        for l2 in (l1 + 1)..lines.len() {
            let m = single_diff(&lines[l1], &lines[l2]);
            if m >= 0 {
                let (mut x, y) = lines[l1].split_at(m as usize);
                let mut result = x.to_string();
                result.push_str(&y[1..]);
                return result;
            }
        }
    }
    "none!".to_string()
}

pub fn run() -> (String, String) {
    let lines = lines::lineread("puzzle_data/day02.txt".to_string());
    (part1(&lines).to_string(), part2(&lines).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vec_of_strings {
        ($($x:expr),*) => (vec![$($x.to_string()),*]);
    }

    #[test]
    fn day02_test1() {
        let v =
            vec_of_strings!["abcdef", "bababc", "abbcde", "abcccd", "aabcdd", "abcdee", "ababab"];
        assert_eq!(part1(&v), 12);
    }

    #[test]
    fn day02_test2() {
        let v = vec_of_strings!["abcde", "fghij", "klmno", "pqrst", "fguij", "axcye", "wvxyz"];
        assert_eq!(part2(&v), "fgij".to_string());
    }
}
