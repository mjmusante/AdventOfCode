use regex::Regex;
use std::collections::HashMap;

use lines;

#[derive(PartialEq, Debug)]
enum Stage {
    TimeStart,
    TimeEnd,
}

fn parts1and2(lines: &Vec<String>) -> (u64, u64) {
    let reg = Regex::new(r"^\[1518-(\d+)-(\d+) (\d+):(\d+)\] (\S+) (\S+)").unwrap();
    let mut curstage = Stage::TimeStart;
    let mut curguard = 0u64;
    let mut starttime = 0u64;
    let mut hm = HashMap::new();

    for l in lines {
        let foo = reg.captures_iter(l).next().unwrap();
        let which = foo[5].to_string();
        if curstage == Stage::TimeStart {
            if which == "Guard" {
                // new guard entry
                curguard = foo[6][1..].parse::<u64>().unwrap();
                if !hm.contains_key(&curguard) {
                    hm.insert(curguard, [0u64; 60]);
                }
                continue;
            }
            // new start entry
            assert_eq!(which, "falls");
            curstage = Stage::TimeEnd;
            starttime = foo[4].parse::<u64>().unwrap();
        } else {
            assert_eq!(curstage, Stage::TimeEnd);
            assert_eq!(which, "wakes");
            let endtime = foo[4].parse::<u64>().unwrap();
            for t in starttime..endtime {
                hm.get_mut(&curguard).unwrap()[t as usize] += 1;
            }
            curstage = Stage::TimeStart;
        }
    }

    let mut maxguard = 0u64;
    let mut maxsleep = 0u64;
    let mut maxtime = 0u64;

    let mut freqguard = 0u64;
    let mut freqmin = 0u64;
    let mut bestfreq = 0u64;

    for (k, v) in hm {
        let mut tcount = 0;
        let mut tfreq = 0;
        let mut tmin = 0;
        for (i, t) in v.iter().enumerate() {
            if *t > 0 {
                tcount += *t;
                if *t > tfreq {
                    tmin = i as u64;
                    tfreq = *t;
                }
                if *t > bestfreq {
                    bestfreq = *t;
                    freqguard = k;
                    freqmin = i as u64;
                }
            }
        }
        if maxsleep < tcount {
            maxsleep = tcount;
            maxtime = tmin;
            maxguard = k;
        }
    }

    (maxguard * maxtime, freqguard * freqmin)
}

pub fn run() -> (String, String) {
    let mut v = lines::lineread("puzzle_data/day04.txt".to_string());
    v.sort();
    let (a, b) = parts1and2(&v);
    (a.to_string(), b.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vec_of_strings {
        ($($x:expr),*) => (vec![$($x.to_string()),*]);
    }

    #[test]
    fn day04_test1() {
        let v = vec_of_strings![
            "[1518-11-01 00:00] Guard #10 begins shift",
            "[1518-11-01 00:05] falls asleep",
            "[1518-11-01 00:25] wakes up",
            "[1518-11-01 00:30] falls asleep",
            "[1518-11-01 00:55] wakes up",
            "[1518-11-01 23:58] Guard #99 begins shift",
            "[1518-11-02 00:40] falls asleep",
            "[1518-11-02 00:50] wakes up",
            "[1518-11-03 00:05] Guard #10 begins shift",
            "[1518-11-03 00:24] falls asleep",
            "[1518-11-03 00:29] wakes up",
            "[1518-11-04 00:02] Guard #99 begins shift",
            "[1518-11-04 00:36] falls asleep",
            "[1518-11-04 00:46] wakes up",
            "[1518-11-05 00:03] Guard #99 begins shift",
            "[1518-11-05 00:45] falls asleep",
            "[1518-11-05 00:55] wakes up"
        ];
        assert_eq!(parts1and2(&v), (240, 4455));
    }
}
