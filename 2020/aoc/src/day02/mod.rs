use aoc::utils::parse_file;
use std::collections::HashMap;

const REXP : &str = r"(?P<min>\d+)-(?P<max>\d+) (?P<ch>.): (?P<passwd>.*)";

pub fn run() {
    let hm = parse_file("data/02.txt", REXP);
    let mut p1valid = 0;
    let mut p2valid = 0;

    for h in hm {
        let (part1, part2) = valid_pwd(&h);
        if part1 {
            p1valid += 1;
        }
        if part2 {
            p2valid += 1
        }
    }

    println!("Part 1 = {}", p1valid);
    println!("Part 2 = {}", p2valid);
}

pub fn valid_pwd(hm: &HashMap<String, String>) -> (bool, bool) {
    let min = hm["min"].parse::<usize>().unwrap();
    let max = hm["max"].parse::<usize>().unwrap();
    let letter = hm["ch"].chars().next().unwrap();
    let mut count = 0;
    let mut pvalid = false;

    for (i, ch) in hm["passwd"].chars().enumerate() {
        if ch == letter {
            count += 1;
            if i + 1 == min || i + 1 == max {
                pvalid = !pvalid;
            }
        }
    }

    (count >= min && count  <= max, pvalid)
}

#[cfg(test)]
mod test {
    use super::*;
    use aoc::utils::parse_vector;

    #[test]
    fn test1() {
        let v = vec!["1-3 a: abcde".to_string()];
        assert_eq!(valid_pwd(&parse_vector(&v, REXP)[0]), (true, true));
    }

    #[test]
    fn test2() {
        let v = vec!["1-3 b: cdefg".to_string()];
        assert_eq!(valid_pwd(&parse_vector(&v, REXP)[0]), (false, false));
    }

    #[test]
    fn test3() {
        let v = vec!["2-9 c: ccccccccc".to_string()];
        assert_eq!(valid_pwd(&parse_vector(&v, REXP)[0]), (true, false));
    }
}
