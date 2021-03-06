use std::collections::HashMap;

use lines;

fn part1(lines: &Vec<String>) -> i64 {
    lines.iter().map(|line| line.parse::<i64>().unwrap()).sum()
}

fn part2(lines: &Vec<String>) -> i64 {
    let mut hm = HashMap::new();
    let mut cur = 0;
    let vals = lines.iter().map(|line| line.parse::<i64>().unwrap());
    hm.insert(cur, 1);
    loop {
        for num in vals.clone() {
            cur += num;
            if hm.contains_key(&cur) {
                return cur;
            }
            hm.insert(cur, 1);
        }
    }
}

pub fn run() -> (String, String) {
    let lines = lines::lineread(String::from("puzzle_data/day01.txt"));
    (part1(&lines).to_string(), part2(&lines).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vec_of_strings {
        ($($x:expr),*) => (vec![$($x.to_string()),*]);
    }

    #[test]
    fn day01_test1() {
        let v = vec_of_strings!["+1", "-2", "+3", "+1"];
        assert_eq!(part1(&v), 3);
    }

    #[test]
    fn day01_test2() {
        let v = vec_of_strings!["+1", "+1", "+1"];
        assert_eq!(part1(&v), 3);
    }

    #[test]
    fn day01_test3() {
        let v = vec_of_strings!["+1", "+1", "-2"];
        assert_eq!(part1(&v), 0);
    }

    #[test]
    fn day01_test4() {
        let v = vec_of_strings!["-1", "-2", "-3"];
        assert_eq!(part1(&v), -6);
    }

    #[test]
    fn day01_test5() {
        let v = vec_of_strings!["+1", "-2", "+3", "+1"];
        assert_eq!(part2(&v), 2);
    }

    #[test]
    fn day01_test6() {
        let v = vec_of_strings!["-1", "+1"];
        assert_eq!(part2(&v), 0);
    }

    #[test]
    fn day01_test7() {
        let v = vec_of_strings!["+3", "+3", "+4", "-2", "-4"];
        assert_eq!(part2(&v), 10);
    }

    #[test]
    fn day01_test8() {
        let v = vec_of_strings!["-6", "+3", "+8", "+5", "-6"];
        assert_eq!(part2(&v), 5);
    }

    #[test]
    fn day01_test9() {
        let v = vec_of_strings!["+7", "+7", "-2", "-7", "-4"];
        assert_eq!(part2(&v), 14);
    }
}
