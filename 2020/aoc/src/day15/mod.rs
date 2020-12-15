use std::collections::HashMap;

pub fn run() {
    let nums = parse(&"14,1,17,0,3,20".to_string());

    println!("Part 1 = {}", num_2020(&nums));
    println!("Part 2 = {}", iterate(&nums, 30_000_000));
}

fn parse(l: &String) -> Vec<i64> {
    l.split(",")
        .map(|num| num.parse().expect("Invalid input"))
        .collect()
}

fn iterate(n: &Vec<i64>, max: i64) -> i64 {
    let mut hist = HashMap::<i64, i64>::new();
    for i in 0..n.len() {
        hist.insert(n[i], i as i64);
    }
    let mut count = n.len() as i64;
    let mut to_insert = 0;
    while count < max - 1 {
        if hist.contains_key(&to_insert) {
            let last_time = *hist.get(&to_insert).unwrap();
            hist.insert(to_insert, count);
            to_insert = count - last_time;
        } else {
            hist.insert(to_insert, count);
            to_insert = 0;
        }
        count += 1;
    }

    to_insert
}

fn num_2020(n: &Vec<i64>) -> i64 {
    iterate(n, 2020)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let nums = parse(&"0,3,6".to_string());
        assert_eq!(num_2020(&nums), 436);
        // assert_eq!(iterate(&nums, 30000000), 175594);
    }

    #[test]
    fn test2() {
        let n1 = parse(&"1,3,2".to_string());
        assert_eq!(num_2020(&n1), 1);
        let n2 = parse(&"2,1,3".to_string());
        assert_eq!(num_2020(&n2), 10);
        let n3 = parse(&"1,2,3".to_string());
        assert_eq!(num_2020(&n3), 27);
        let n4 = parse(&"2,3,1".to_string());
        assert_eq!(num_2020(&n4), 78);
        let n5 = parse(&"3,2,1".to_string());
        assert_eq!(num_2020(&n5), 438);
        let n6 = parse(&"3,1,2".to_string());
        assert_eq!(num_2020(&n6), 1836);
    }
}
