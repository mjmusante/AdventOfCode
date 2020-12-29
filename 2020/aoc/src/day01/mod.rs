use itertools::Itertools;
use std::collections::HashSet;
use std::iter::FromIterator;

use aoc::utils::set_of_nums;

pub fn run() {
    let hs = set_of_nums("data/01.txt");

    println!("Part 1 = {}", prod_sum(&hs, 2).unwrap());
    println!("Part 2 = {}", prod_sum(&hs, 3).unwrap());
}

fn prod_sum(hs: &HashSet<i64>, n: usize) -> Option<i64> {
    let iv = hs.iter().cloned().combinations(n - 1);

    for inverse in iv {
        let tot: i64 = inverse.iter().sum();
        if hs.contains(&(2020 - tot)) {
            return Some(inverse.iter().fold(2020 - tot, |prod, &mul| prod * mul));
        }
    }

    None
}

#[cfg(test)]
mod test {
    use super::*;

    fn data() -> HashSet<i64> {
        let v: Vec<i64> = vec![1721, 979, 366, 299, 675, 1456];
        HashSet::from_iter(v.iter().cloned())
    }

    #[test]
    fn test1() {
        assert_eq!(prod_sum(&data(), 2).unwrap(), 514579);
    }

    #[test]
    fn test2() {
        assert_eq!(prod_sum(&data(), 3).unwrap(), 241861950);
    }
}
