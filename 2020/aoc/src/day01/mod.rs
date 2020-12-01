use itertools::Itertools;

use aoc::utils::nums;

pub fn day01() {
    let v = nums("data/01.txt");
    for pair in v.clone().into_iter().combinations(2) {
        if pair[0] + pair[1] == 2020 {
            println!("Part 1 = {}", pair[0] * pair[1]);
            break;
        }
    }

    for pair in v.clone().into_iter().combinations(3) {
        if pair[0] + pair[1] + pair[2] == 2020 {
            println!("Part 2 = {}", pair[0] * pair[1] * pair[2]);
            break;
        }
    }
}
