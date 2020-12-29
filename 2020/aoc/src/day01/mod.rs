use itertools::Itertools;
use std::collections::HashSet;
use std::iter::FromIterator;

use aoc::utils::nums;

pub fn run() {
    let v = nums("data/01.txt");

    println!("Part 1 = {}", prod_sum(&v, 2));
    println!("Part 2 = {}", prod_sum(&v, 3));
}

fn prod_sum(v: &Vec<i64>, n: usize) -> i64 {
    let hs : HashSet::<i64> = HashSet::from_iter(v.iter().cloned());
    let iv = hs.iter().cloned().combinations(n - 1);

    for inverse in iv {
        let tot : i64 = inverse.iter().sum();
        if hs.contains(&(2020 - tot)) {
            return inverse.iter().fold(2020 - tot, |prod, &mul| prod *  mul);
        }
    }

    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let v: Vec<i64> = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(prod_sum(&v, 2), 514579);
    }

    #[test]
    fn test2() {
        let v: Vec<i64> = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(prod_sum(&v, 3), 241861950);
    }
}
