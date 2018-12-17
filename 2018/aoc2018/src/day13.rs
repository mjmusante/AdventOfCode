use lines;

pub fn run() -> (String, String) {
    let lines = lines::lineread("puzzle_data/day13.txt".to_string());

    let mut t = Track::new(&lines);
    let part1ans: (usize, usize);
    let part2ans: (usize, usize);
    loop {
        let (part1, _) = t.tick();
        if part1.is_dead {
            part1ans = part1.pos;
            break;
        }
    }
    loop {
        let (_, part2) = t.tick();
        if part2.is_dead {
            part2ans = part2.pos;
            break;
        }
    }

    (format!("{:?}", part1ans), format!("{:?}", part2ans))
}

// --------------------------------

use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
enum Rotate {
    Left,
    Straight,
    Right,
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

struct Cart {
    pos: (usize, usize),
    dir: Direction,
    turn: Rotate,
}

struct Track {
    grid: Vec<Vec<char>>,
    cart: Vec<Cart>,
}

struct Death {
    is_dead: bool,
    pos: (usize, usize),
}

impl Track {
    pub fn new(input: &Vec<String>) -> Track {
        let mut grid: Vec<Vec<char>> = vec![];
        for l in input {
            grid.push(
                l.chars()
                    .map(|x| match x {
                        'v' | '^' => '|',
                        '<' | '>' => '-',
                        _ => x,
                    }).collect(),
            );
        }
        let mut cart = vec![];
        for row in 0..input.len() {
            for i in input[row]
                .chars()
                .enumerate()
                .filter(|(_, c)| *c == 'v' || *c == '^' || *c == '<' || *c == '>')
                .map(|(i, c)| ((row, i), c))
            {
                cart.push(Cart {
                    pos: i.0,
                    dir: match i.1 {
                        '^' => Direction::North,
                        'v' => Direction::South,
                        '>' => Direction::East,
                        '<' => Direction::West,
                        _ => panic!("wtf"),
                    },
                    turn: Rotate::Left,
                });
            }
        }
        Track {
            grid: grid,
            cart: cart,
        }
    }

    pub fn tick(&mut self) -> (Death, Death) {
        let mut hm = HashMap::new();
        let mut dead = HashMap::new();
        let mut first_dead = Death {
            is_dead: false,
            pos: (0, 0),
        };
        let mut last_dead = Death {
            is_dead: false,
            pos: (0, 0),
        };

        self.cart.sort_by(|a, b| {
            if a.pos.0 < b.pos.0 {
                Ordering::Less
            } else if a.pos.0 > b.pos.0 {
                Ordering::Greater
            } else if a.pos.1 < b.pos.1 {
                Ordering::Less
            } else if a.pos.1 > b.pos.1 {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });

        for c in &self.cart {
            if hm.contains_key(&c.pos) {
                if !first_dead.is_dead {
                    first_dead = Death {
                        is_dead: true,
                        pos: (c.pos.1, c.pos.0),
                    };
                }
                dead.insert(c.pos.clone(), 1);
                continue;
            }
            let nextpos = match c.dir {
                Direction::North => (c.pos.0 - 1, c.pos.1),
                Direction::South => (c.pos.0 + 1, c.pos.1),
                Direction::East => (c.pos.0, c.pos.1 + 1),
                Direction::West => (c.pos.0, c.pos.1 - 1),
            };
            // println!("Working ({},{}) moving to ({},{})", c.pos.0, c.pos.1, nextpos.0, nextpos.1);

            let (newturn, newdir) = match self.grid[nextpos.0][nextpos.1] {
                '/' => match c.dir {
                    Direction::North => (c.turn, Direction::East),
                    Direction::South => (c.turn, Direction::West),
                    Direction::East => (c.turn, Direction::North),
                    Direction::West => (c.turn, Direction::South),
                },
                '\\' => match c.dir {
                    Direction::North => (c.turn, Direction::West),
                    Direction::South => (c.turn, Direction::East),
                    Direction::East => (c.turn, Direction::South),
                    Direction::West => (c.turn, Direction::North),
                },
                '|' | '-' => (c.turn, c.dir.clone()),
                '+' => Track::crossroads(&c.dir, &c.turn),
                _ => panic!("wtf"),
            };

            let newc = Cart {
                pos: nextpos,
                dir: newdir,
                turn: newturn,
            };

            if hm.contains_key(&nextpos) {
                if !first_dead.is_dead {
                    first_dead = Death {
                        is_dead: true,
                        pos: (nextpos.1, nextpos.0),
                    };
                }
                dead.insert(nextpos.clone(), 1);
            } else {
                hm.insert(nextpos, newc);
            }

            // println!("newturn {:?}, newdir {:?}, newpos {:?}", newturn, newdir, nextpos);
        }

        self.cart.clear();
        for (_, v) in hm {
            if !dead.contains_key(&v.pos) {
                self.cart.push(v);
            }
        }

        if self.cart.len() == 1 {
            last_dead = Death {
                is_dead: true,
                pos: (self.cart[0].pos.1, self.cart[0].pos.0),
            };
        }

        // println!("-");
        (first_dead, last_dead)
    }

    fn turn_left(d: &Direction) -> Direction {
        match d {
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
            Direction::West => Direction::South,
        }
    }

    fn turn_right(d: &Direction) -> Direction {
        match d {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
        }
    }

    fn crossroads(d: &Direction, t: &Rotate) -> (Rotate, Direction) {
        match t {
            Rotate::Left => (Rotate::Straight, Track::turn_left(d)),
            Rotate::Straight => (Rotate::Right, *d),
            Rotate::Right => (Rotate::Left, Track::turn_right(d)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vec_of_strings {
        ($($x:expr),*) => (vec![$($x.to_string()),*]);
    }

    #[test]
    fn day13_test1() {
        let t = Track::new(&vec_of_strings!["|", "v", "|", "|", "|", "^", "|"]);
        assert_eq!(t.cart[0].pos, (1, 0));
        assert_eq!(t.cart[1].pos, (5, 0));
    }

    #[test]
    fn day13_test2() {
        let mut t = Track::new(&vec_of_strings!["|", "v", "|", "|", "|", "^", "|"]);
        t.tick();
        let (d, _) = t.tick();
        assert_eq!(d.is_dead, true);
        assert_eq!(d.pos, (0, 3));
    }

    #[test]
    fn day13_test3() {
        let mut t = Track::new(&vec_of_strings![
            "/->-\\        ",
            "|   |  /----\\",
            "| /-+--+-\\  |",
            "| | |  | v  |",
            "\\-+-/  \\-+--/",
            "  \\------/   "
        ]);
        loop {
            let (result, _) = t.tick();
            if result.is_dead {
                assert_eq!(result.pos, (7, 3));
                break;
            }
        }
    }

    #[test]
    fn day13_test4() {
        let mut t = Track::new(&vec_of_strings![
            "/>-<\\  ", "|   |  ", "| /<+-\\", "| | | v", "\\>+</ |", "  |   ^", "  \\<->/"
        ]);
        loop {
            let (_, result) = t.tick();
            if result.is_dead {
                assert_eq!(result.pos, (6, 4));
                break;
            }
        }
    }
}
