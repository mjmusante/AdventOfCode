use aoc::utils::lines;

use regex::Regex;

pub fn day02() {
    let reg = Regex::new(r"(\d+)-(\d+) (.): (.*)").unwrap();
    if check(&reg, "1-3 a: abcde") != (true, true) {
        panic!("failure in case 1");
    }
    if check(&reg, "1-3 b: cdefg") != (false, false) {
        panic!("failure in case 2");
    }
    if check(&reg, "2-9 c: ccccccccc") != (true, false) {
        panic!("failure in case 3");
    }

    let mut p1valid = 0;
    let mut p2valid = 0;
    let data = lines("data/02.txt");
    for d in data {
        let (part1, part2) = check(&reg, &d);
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

fn check(reg: &Regex, line: &str) -> (bool, bool) {
    let foo = reg.captures_iter(line).next().unwrap();
    let low = foo[1].parse::<i64>().unwrap();
    let high = foo[2].parse::<i64>().unwrap();
    let letter = foo[3].to_string().chars().next().unwrap();
    let passwd = foo[4].to_string();

    let mut count = 0;
    let mut pvalid = false;
    let mut loc = 1;
    for n in passwd.chars() {
        if n == letter {
            count += 1;
            if loc == low || loc == high {
                pvalid = !pvalid;
            }
        }
        loc += 1;
    }

    (count >= low && count <= high, pvalid)
}
