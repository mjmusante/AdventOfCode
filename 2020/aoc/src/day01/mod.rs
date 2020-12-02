use itertools::Itertools;

use aoc::utils::nums;

pub fn run() {
    let v = nums("data/01.txt");
    println!(
        "Part 1 = {}",
        v.clone()
            .into_iter()
            .combinations(2)
            .filter(|x| x[0] + x[1] == 2020)
            .nth(0)
            .unwrap()
            .iter()
            .fold(1, |prod, &mul| prod * mul)
    );

    println!(
        "Part 2 = {}",
        v.into_iter()
            .combinations(3)
            .filter(|x| x[0] + x[1] + x[2] == 2020)
            .nth(0)
            .unwrap()
            .iter()
            .fold(1, |prod, &mul| prod * mul)
    );
}
