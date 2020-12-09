use itertools::Itertools;

use aoc::utils::nums;

pub fn run() {
    let num = nums("data/09.txt");
    println!("Part 1 = {}", find_first(25, &num));
}

fn find_first(pre: usize, v: &Vec<i64>) -> i64 {
    for i in pre..v.len() {
        let f = v.get(i).unwrap();
        let m = &v[(i - pre)..i];
        let n = m.iter().combinations(2).map(|x| x[0] + x[1]).collect::<Vec<i64>>();
        if !n.contains(f) {
            return *f;
        }
    }

    -1
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_data() -> Vec<i64> {
        vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ]
    }

    #[test]
    fn test1() {
        assert_eq!(find_first(5, &test_data()), 127);
    }
}
