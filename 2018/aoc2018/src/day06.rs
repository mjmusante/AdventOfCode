use lines;
use std::cmp::{max, min};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
struct GridEntry {
    name: (i64, i64),
    distance: u64,
    equal: bool,
}

fn solve(lines: &Vec<String>, region: u64) -> (String, String) {
    let coords = lines
        .iter()
        .map(|str| str.split(", "))
        .map(|mut coord| {
            (
                coord.next().unwrap().parse::<i64>().unwrap(),
                coord.next().unwrap().parse::<i64>().unwrap(),
            )
        }).collect::<Vec<(i64, i64)>>();
    let mut i = coords.iter();
    let first = i.next().unwrap();
    let mut min_x = first.0;
    let mut min_y = first.1;
    let mut max_x = 0i64;
    let mut max_y = 0i64;
    for c in i {
        min_x = min(c.0, min_x);
        min_y = min(c.1, min_y);
        max_x = max(c.0, max_x);
        max_y = max(c.1, max_y);
    }
    let width = max_x - min_x;
    let height = max_y - min_y;
    let maxdist = (width * height) as u64;
    let mut grid = vec![
        vec![
            GridEntry {
                name: (0, 0),
                distance: maxdist,
                equal: false
            };
            height as usize
        ];
        width as usize
    ];
    let mut rcount = 0;
    for x in 0..width {
        for y in 0..height {
            let mut total = 0;
            for c in coords.iter() {
                let old_dist = grid[x as usize][y as usize].distance;
                let dist = ((c.0 - x - min_x).abs() + (c.1 - y - min_y).abs()) as u64;
                total += dist;
                if old_dist > dist {
                    grid[x as usize][y as usize] = GridEntry {
                        name: *c,
                        distance: dist,
                        equal: false,
                    };
                } else if old_dist == dist {
                    grid[x as usize][y as usize].equal = true;
                }
            }
            if total < region {
                rcount += 1;
            }
        }
    }
    let mut hm = HashMap::new();
    for e in grid
        .iter()
        .flatten()
        .filter(|y| y.distance < maxdist && !y.equal)
    {
        *hm.entry(e.name).or_insert(0) += 1;
    }
    (hm.values().max().unwrap().to_string(), rcount.to_string())
}

pub fn run() -> (String, String) {
    let lines = lines::lineread(String::from("puzzle_data/day06.txt"));

    solve(&lines, 10000)
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vec_of_strings {
        ($($x:expr),*) => (vec![$($x.to_string()),*]);
    }

    #[test]
    fn day06_part1() {
        let v = vec_of_strings!["1, 1", "1, 6", "8, 3", "3, 4", "5, 5", "8, 9"];
        assert_eq!(solve(&v, 32), ("17".to_string(), "16".to_string()));
    }
}
