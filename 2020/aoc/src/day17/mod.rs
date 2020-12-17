use std::collections::HashSet;

pub fn run() {}

type Location = (i64, i64, i64);

struct Universe {
    state: HashSet<Location>,
    surroundings: Vec<Location>
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

        Universe { state, surroundings }
    }

    pub fn step(&mut self) {
        let mut newstate = HashSet::<Location>::new();
        for loc in &self.state {
            let mut num = 0;
            for s in &self.surroundings {
                let check = (loc.0 + s.0, loc.1 + s.1, loc.2 + s.2);
                if self.state.contains(&check) {
                    num += 1;
                }
            }
            if num == 2 || num == 3 {
                newstate.insert(*loc);
            }
            // TBD!
        }

        self.state = newstate;
    }

    pub fn count_active(&self) -> i64 {
        self.state.len() as i64
    }
}

fn parse(lines: &Vec<String>) -> Universe {
    let mut u = Universe::new();

    let mut x = 0;
    for l in lines {
        let mut y = 0;
        for c in l.chars() {
            match c {
                '#' => { u.state.insert((x, y, 0)); },
                '.' => (),
                _ => panic!("invalid char in input"),
            }
            y += 1;
        }
        x += 1;
    }

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
}
