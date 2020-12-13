use aoc::utils::lines;
use aoc::utils::mod_inv;

pub fn run() {
    let data = parse(&lines("data/13.txt"));

    println!("Part 1 = {}", next_bus(&data));
    println!("Part 2 = {}", contest(&data));
}

#[derive(Debug)]
struct Schedule {
    timestamp: i128,
    bus: Vec<i128>,
}

fn parse(lines: &Vec<String>) -> Schedule {
    let mut sched = Schedule {
        timestamp: 0,
        bus: Vec::new(),
    };

    sched.timestamp = lines.get(0).unwrap().parse().expect("invalid timeval");

    sched.bus = lines
        .get(1)
        .unwrap()
        .split(",")
        .map(|val| match val {
            "x" => 0,
            val => val.parse().expect("invalid bus id"),
        })
        .collect();

    sched
}

fn next_bus(sched: &Schedule) -> i128 {
    let wait_times: Vec<(i128, i128)> = sched
        .bus
        .iter()
        .filter(|x| **x != 0)
        .map(|x| (*x, x - sched.timestamp % x))
        .collect();

    let mut min = wait_times[0];
    for w in 1..wait_times.len() {
        if min.1 > wait_times[w].1 {
            min = wait_times[w];
        }
    }

    min.0 * min.1
}

fn contest(sched: &Schedule) -> i128 {
    let mut list = Vec::new();
    let mut x = 0;
    let mut prod = 1;
    for i in &sched.bus {
        if *i > 0 {
            list.push((i - x % i, i));
            prod *= i;
        }
        x += 1;
    }

    // Chinese Remainder 
    // Translated from https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust
    let mut sum = 0;
    for i in list {
        if i.0 == 0 || i.0 == *i.1 { continue; }
        let p = prod / i.1;
        let mi = mod_inv(p, *i.1).expect("no modular inverse");
        sum += i.0 * mi * p;
    }

    sum % prod
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_data() -> Vec<String> {
        vec!["939".to_string(), "7,13,x,x,59,x,31,19".to_string()]
    }

    #[test]
    fn test1() {
        let v = parse(&test_data());
        assert_eq!(next_bus(&v), 295);
    }

    #[test]
    fn test2() {
        let v = parse(&test_data());
        assert_eq!(contest(&v), 1068781);
    }
}
