use itertools::Itertools;

use aoc::utils::nums;

pub fn run() {
    let v = nums("data/01.txt");

    println!("Part 1 = {}", part1(&v));
    println!("Part 2 = {}", part2(&v));
}

fn part1(v: &Vec<i64>) -> i64 {
    v.iter()
        .combinations(2)
        .filter(|x| x[0] + x[1] == 2020)
        .nth(0)
        .unwrap()
        .iter()
        .fold(1, |prod, &mul| prod * mul)
}

fn part2(v: &Vec<i64>) -> i64 {
    v.iter()
        .combinations(3)
        .filter(|x| x[0] + x[1] + x[2] == 2020)
        .nth(0)
        .unwrap()
        .iter()
        .fold(1, |prod, &mul| prod * mul)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let v: Vec<i64> = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(part1(&v), 514579);
    }

    #[test]
    fn test2() {
        let v: Vec<i64> = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(part2(&v), 241861950);
    }
}
