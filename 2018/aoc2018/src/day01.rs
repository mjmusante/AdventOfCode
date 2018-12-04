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

pub fn run() {
    let lines = lines::lineread(String::from("puzzle_data/day01.txt"));
    println!("Part 1: {}", part1(&lines));
    println!("Part 2: {}", part2(&lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vec_of_strings {
        ($($x:expr),*) => (vec![$($x.to_string()),*]);
    }

    #[test]
    fn test1() {
        let v = vec_of_strings!["+1", "-2", "+3", "+1"];
        assert_eq!(part1(&v), 3);
    }

    #[test]
    fn test2() {
        let v = vec_of_strings!["+1", "+1", "+1"];
        assert_eq!(part1(&v), 3);
    }

    #[test]
    fn test3() {
        let v = vec_of_strings!["+1", "+1", "-2"];
        assert_eq!(part1(&v), 0);
    }

    #[test]
    fn test4() {
        let v = vec_of_strings!["-1", "-2", "-3"];
        assert_eq!(part1(&v), -6);
    }

    #[test]
    fn test5() {
        let v = vec_of_strings!["+1", "-2", "+3", "+1"];
        assert_eq!(part2(&v), 2);
    }

    #[test]
    fn test6() {
        let v = vec_of_strings!["-1", "+1"];
        assert_eq!(part2(&v), 0);
    }

    #[test]
    fn test7() {
        let v = vec_of_strings!["+3", "+3", "+4", "-2", "-4"];
        assert_eq!(part2(&v), 10);
    }

    #[test]
    fn test8() {
        let v = vec_of_strings!["-6", "+3", "+8", "+5", "-6"];
        assert_eq!(part2(&v), 5);
    }

    #[test]
    fn test9() {
        let v = vec_of_strings!["+7", "+7", "-2", "-7", "-4"];
        assert_eq!(part2(&v), 14);
    }
}
