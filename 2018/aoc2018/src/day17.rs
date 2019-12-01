use lines;

pub fn run() -> (String, String) {
    let lines = lines::lineread(String::from("puzzle_data/day17.txt"));

    let g = Ground::new(lines);
    g.display();
    ("hello".to_string(), "world".to_string())
}

// ------

use std::collections::HashMap;
use regex::Regex;
use std::cmp::{max, min};

struct Ground {
    minx: usize,
    miny: usize,
    maxx: usize,
    maxy: usize,
    ground: HashMap<(usize, usize), char>
}

impl Ground {
    fn set_minmax(&mut self, sx: usize, bx: usize, by: usize) {
        self.minx = min(sx, self.minx);
        self.maxx = max(bx, self.maxx);
        self.maxy = max(by, self.maxy);
    }

    pub fn new(lines: Vec<String>) -> Ground {
        let mut g = Ground { minx: <usize>::max_value(), miny: 0, maxx: 0, maxy: 0, ground: HashMap::new() };
        let reg = Regex::new(r"^(.)=(\d+), .=(\d+)\.\.(\d+)$").unwrap();

        g.ground.insert((500, 0), '+');

        for l in lines {
            let c = reg.captures_iter(&l).next().unwrap();
            let rc = c[2].parse::<usize>().unwrap(); // row or column
            let start = c[3].parse::<usize>().unwrap(); // start of range
            let end = c[4].parse::<usize>().unwrap();

            match &c[1] {
                "x" => {
                    // vertical range
                    for y in start..=end {
                        g.ground.insert((rc, y), '#');
                    }
                    g.set_minmax(rc, rc, end);
                }
                "y" => {
                    // horizontal range
                    for x in start..=end {
                        g.ground.insert((x, rc), '#');
                    }
                    g.set_minmax(start, end, rc);
                }
                _ => { panic!("invalid line"); }
            }
        }

        g.minx -= 1;
        g.maxx += 1;

        g
    }

    pub fn display(&self) {
        println!("x range is {}..{}", self.minx, self.maxx);
        for y in self.miny..=self.maxy {
            for x in self.minx..=self.maxx {
                if self.ground.contains_key(&(x, y)) {
                    print!("{}", self.ground.get(&(x, y)).unwrap());
                } else {
                    print!(".");
                }
            }
            println!(" <-- {}", y);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day17_test1() {
        let v = [
            String::from("x=495, y=2..7"),
            String::from("y=7, x=495..501"),
            String::from("x=501, y=3..7"),
            String::from("x=498, y=2..4"),
            String::from("x=506, y=1..2"),
            String::from("x=498, y=10..13"),
            String::from("x=504, y=10..13"),
            String::from("y=13, x=498..504"),
        ].to_vec();

        let m = Ground::new(v);
        m.display();
        assert_eq!(m.minx, 494);
    }
}
