use aoc::utils::lines;

pub fn run() {
    let data = lines("data/13.txt");

    println!("Part 1 = {}", next_bus(&parse(&data)));
}

#[derive(Debug)]
struct Schedule {
    timestamp: i64,
    bus: Vec<i64>,
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

fn next_bus(sched: &Schedule) -> i64 {
    let wait_times: Vec<(i64, i64)> = sched
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

#[cfg(test)]
mod test {
    use super::*;

    fn test_data() -> Vec<String> {
        vec!["939".to_string(), "7,13,x,x,59,x,31,19".to_string()]
        // vec!["1007153".to_string(),
        //      "29,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,37,x,x,x,x,x,433,x,x,x,x,x,x,x,x,x,x,x,x,13,17,x,x,x,x,19,x,x,x,23,x,x,x,x,x,x,x,977,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,41".to_string()]
    }

    #[test]
    fn test1() {
        let v = parse(&test_data());
        assert_eq!(next_bus(&v), 295);
    }
}
