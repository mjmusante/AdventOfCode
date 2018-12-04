use lines;

use regex::Regex;
use std::cmp::{min, max};
use std::collections::HashMap;
use itertools::Itertools;

#[derive(PartialEq, Eq, Hash)]
struct Region {
    id: u64,
    xpos: u64,
    ypos: u64,
    width: u64,
    height: u64
}

fn part1(lines: &Vec<Region>) -> (u64, u64) {
    let mut hm = HashMap::new();
    let mut noverlap = HashMap::new();

    for l in lines {
        noverlap.insert(l, 0);
    }

    for (r1, r2) in lines.iter().tuple_combinations() {
        let r1x = r1.xpos + r1.width - 1;
        let r1y = r1.ypos + r1.height - 1;
        let r2x = r2.xpos + r2.width - 1;
        let r2y = r2.ypos + r2.height - 1;

        if (r1.xpos < r2.xpos && r1x < r2.xpos) ||
            (r2.xpos < r1.xpos && r2x < r1.xpos) {
                continue;
            }
        if (r1.ypos < r2.ypos && r1y < r2.ypos) ||
            (r2.ypos < r1.ypos && r2y < r1.ypos) {
                continue;
            }

        // they overlap
        noverlap.remove(r1);
        noverlap.remove(r2);

        let ox = max(r1.xpos, r2.xpos);
        let oy = max(r1.ypos, r2.ypos);
        let px = min(r1x, r2x) + 1;
        let py = min(r1y, r2y) + 1;

        for x in ox..px {
            for y in oy..py {
                *hm.entry((x, y)).or_insert(0) += 1;
            }
        }
    }
    (hm.keys().len() as u64, noverlap.keys().next().unwrap().id)
}

fn convert(lines: &Vec<String>) -> Vec<Region> {
    let reg = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();
    let mut claims: Vec<Region> = vec![];

    for l in lines {
        let cap = reg.captures_iter(l).next().unwrap();
        claims.push(Region {
            id: cap[1].parse::<u64>().unwrap(),
            xpos: cap[2].parse::<u64>().unwrap(),
            ypos: cap[3].parse::<u64>().unwrap(),
            width: cap[4].parse::<u64>().unwrap(),
            height: cap[5].parse::<u64>().unwrap()
        });
    }

    claims
}

pub fn run() {
    let lines = lines::lineread("puzzle_data/day03.txt".to_string());
    let c = convert(&lines);
    let (area, safeid) = part1(&c);


    println!("Part 1: {}", area);
    println!("Part 2: {}", safeid);
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vec_of_strings {
        ($($x:expr),*) => (vec![$($x.to_string()),*]);
    }

    #[test]
    fn it_works() {
        let v = vec_of_strings!["#1 @ 1,3: 4x4", "#2 @ 3,1: 4x4", "#3 @ 5,5: 2x2" ];
        let c = convert(&v);
        let (area, safeid) = part1(&c);

        assert_eq!(area, 4);
        assert_eq!(safeid, 3);
    }
}
