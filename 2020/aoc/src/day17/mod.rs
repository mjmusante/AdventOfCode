use std::collections::HashSet;

use aoc::utils::lines;

pub fn run() {
    let lines = lines("data/17.txt");

    let mut universe = parse(&lines);
    for _ in 0..6 {
        universe.step();
    }
    println!("Part 1 = {}", universe.count_active());

    let mut universe4 = parse4(&lines);
    for _ in 0..6 {
        universe4.step();
    }
    println!("Part 2 = {}", universe4.count_active());
}

type Range = (i64, i64);

type Location = (i64, i64, i64);
type Location4 = (i64, i64, i64, i64);

struct Universe {
    xrange: Range,
    yrange: Range,
    zrange: Range,
    state: HashSet<Location>,
    surroundings: Vec<Location>,
}

struct Universe4 {
    wrange: Range,
    xrange: Range,
    yrange: Range,
    zrange: Range,
    state: HashSet<Location4>,
    surroundings: Vec<Location4>,
}

impl Universe {
    pub fn new() -> Universe {
        let mut surroundings = Vec::<Location>::new();
        let state = HashSet::<Location>::new();

        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    if x == 0 && y == 0 && z == 0 {
                        continue;
                    }
                    surroundings.push((x, y, z));
                }
            }
        }

        Universe {
            xrange: (0, 0),
            yrange: (0, 0),
            zrange: (0, 0),
            state,
            surroundings,
        }
    }

    pub fn step(&mut self) {
        let mut newstate = HashSet::<Location>::new();
        let xn = (self.xrange.0 - 1, self.xrange.1 + 1);
        let yn = (self.yrange.0 - 1, self.yrange.1 + 1);
        let zn = (self.zrange.0 - 1, self.zrange.1 + 1);

        for x in xn.0..=xn.1 {
            for y in yn.0..=yn.1 {
                for z in zn.0..=zn.1 {
                    let loc = (x, y, z);
                    let active = self.state.contains(&loc);
                    let mut num = 0;

                    for s in &self.surroundings {
                        let check = (loc.0 + s.0, loc.1 + s.1, loc.2 + s.2);
                        if self.state.contains(&check) {
                            num += 1;
                        }
                    }
                    if (active && (num == 2 || num == 3)) || (!active && num == 3) {
                        newstate.insert(loc);
                    }
                }
            }
        }

        // println!("-- step --");
        // for z in zn.0..=zn.1 {
        //     println!("z = {}", z);
        //     for y in yn.0..=yn.1 {
        //         for x in xn.0..=xn.1 {
        //             print!(
        //                 "{}",
        //                 if newstate.contains(&(x, y, z)) {
        //                     "#"
        //                 } else {
        //                     "."
        //                 }
        //             );
        //         }
        //         println!("");
        //     }
        // }

        self.xrange = xn;
        self.yrange = yn;
        self.zrange = zn;
        self.state = newstate;
    }

    pub fn count_active(&self) -> i64 {
        self.state.len() as i64
    }
}

impl Universe4 {
    pub fn new() -> Universe4 {
        let mut surroundings = Vec::<Location4>::new();
        for w in -1..=1 {
            for x in -1..=1 {
                for y in -1..=1 {
                    for z in -1..=1 {
                        if w == 0 && x == 0 && y == 0 && z == 0 {
                            continue;
                        }
                        surroundings.push((w, x, y, z));
                    }
                }
            }
        }

        Universe4 {
            wrange: (0, 0),
            xrange: (0, 0),
            yrange: (0, 0),
            zrange: (0, 0),
            state: HashSet::<Location4>::new(),
            surroundings,
        }
    }

    pub fn step(&mut self) {
        let mut newstate = HashSet::<Location4>::new();
        let wn = (self.wrange.0 - 1, self.wrange.1 + 1);
        let xn = (self.xrange.0 - 1, self.xrange.1 + 1);
        let yn = (self.yrange.0 - 1, self.yrange.1 + 1);
        let zn = (self.zrange.0 - 1, self.zrange.1 + 1);

        for w in wn.0..=wn.1 {
            for x in xn.0..=xn.1 {
                for y in yn.0..=yn.1 {
                    for z in zn.0..=zn.1 {
                        let loc = (w, x, y, z);
                        let active = self.state.contains(&loc);
                        let mut num = 0;

                        for s in &self.surroundings {
                            let check = (loc.0 + s.0, loc.1 + s.1, loc.2 + s.2, loc.3 + s.3);
                            if self.state.contains(&check) {
                                num += 1;
                            }
                        }
                        if (active && (num == 2 || num == 3)) || (!active && num == 3) {
                            newstate.insert(loc);
                        }
                    }
                }
            }
        }

        self.wrange = wn;
        self.xrange = xn;
        self.yrange = yn;
        self.zrange = zn;
        self.state = newstate;
    }

    pub fn count_active(&self) -> i64 {
        self.state.len() as i64
    }
}

fn parse(lines: &Vec<String>) -> Universe {
    let mut u = Universe::new();
    let mut max_x = 0;

    let mut y = 0;
    for l in lines {
        let mut x = 0;
        for c in l.chars() {
            match c {
                '#' => {
                    u.state.insert((x, y, 0));
                }
                '.' => (),
                _ => panic!("invalid char in input"),
            }
            x += 1;
        }
        if x > max_x {
            max_x = x;
        }
        y += 1;
    }
    u.xrange = (0, max_x);
    u.yrange = (0, y);

    u
}

fn parse4(lines: &Vec<String>) -> Universe4 {
    let mut u = Universe4::new();
    let mut max_x = 0;

    let mut y = 0;
    for l in lines {
        let mut x = 0;
        for c in l.chars() {
            match c {
                '#' => {
                    u.state.insert((0, x, y, 0));
                }
                '.' => (),
                _ => panic!("invalid char in input"),
            }
            x += 1;
        }
        if x > max_x {
            max_x = x;
        }
        y += 1;
    }
    u.xrange = (0, max_x);
    u.yrange = (0, y);

    u
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let v = vec![".#.".to_string(), "..#".to_string(), "###".to_string()];
        let mut u = parse(&v);
        for _ in 0..6 {
            u.step();
        }
        assert_eq!(u.count_active(), 112);
    }

    #[test]
    #[ignore]
    fn test2() {
        let v = vec![".#.".to_string(), "..#".to_string(), "###".to_string()];
        let mut u = parse4(&v);
        for _ in 0..6 {
            u.step();
        }
        assert_eq!(u.count_active(), 848);
    }
}
