use itertools::Itertools;

use aoc::utils::nums;

pub fn day01() {
    let v = nums("data/01.txt");
    for pair in v
        .clone()
        .into_iter()
        .combinations(2)
        .filter(|x| x[0] + x[1] == 2020)
    {
        println!("Part 1 = {}", pair[0] * pair[1]);
        break;
    }

    for trip in v
        .into_iter()
        .combinations(3)
        .filter(|x| x[0] + x[1] + x[2] == 2020)
    {
        println!("Part 2 = {}", trip[0] * trip[1] * trip[2]);
        break;
    }
}
