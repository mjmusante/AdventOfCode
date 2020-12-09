use itertools::Itertools;

use aoc::utils::nums;

pub fn run() {
    let num = nums("data/09.txt");
    let part1 = find_first(25, &num);
    println!("Part 1 = {}", part1);
    println!("Part 2 = {}", find_sum(&num, part1));
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

fn find_sum(v: &Vec<i64>, target: i64) -> i64 {
    let w = v.clone().into_iter().filter(|x| x < &target).collect::<Vec<i64>>();

    for i in 0..w.len() - 1 {
        for j in i + 1..w.len() {
            let s : i64 = w[i..j].iter().sum();
            if s == target {
                let min = w[i..j].iter().min().expect("No min found");
                let max = w[i..j].iter().max().expect("No max found");
                return min + max;
            }
        }
    }

    w.len() as i64
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

    #[test]
    fn test2() {
        assert_eq!(find_sum(&test_data(), 127), 62);
    }
}
