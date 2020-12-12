use aoc::utils::lines;

pub fn run() {
    let lines = lines("data/12.txt");
    let inst = parse(&lines);

    println!("Part 1 = {}", distance(&inst));
    println!("Part 2 = {}", waypoint(&inst));
}

#[derive(Debug, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

struct Step {
    d: Direction,
    m: i64,
}

fn parse(lines: &Vec<String>) -> Vec<Step> {
    let mut result = Vec::new();
    for l in lines {
        let dir = match l.chars().nth(0).expect("empty string") {
            'N' => Direction::North,
            'S' => Direction::South,
            'E' => Direction::East,
            'W' => Direction::West,
            'L' => Direction::Left,
            'R' => Direction::Right,
            'F' => Direction::Forward,
            _ => {
                panic!("Invalid char in input")
            }
        };
        let amt = &l[1..].parse::<i64>().expect("invalid number");
        result.push(Step { d: dir, m: *amt });
    }

    result
}

fn turn_left(d: &Direction) -> Direction {
    match d {
        Direction::North => Direction::West,
        Direction::West => Direction::South,
        Direction::South => Direction::East,
        Direction::East => Direction::North,
        _ => {
            panic!("bad turn")
        }
    }
}

fn turn_right(d: &Direction) -> Direction {
    match d {
        Direction::North => Direction::East,
        Direction::West => Direction::North,
        Direction::South => Direction::West,
        Direction::East => Direction::South,
        _ => {
            panic!("bad turn")
        }
    }
}

fn rotate_left(x: i64, y: i64) -> (i64, i64) {
    (-y, x)
}

fn rotate_right(x: i64, y: i64) -> (i64, i64) {
    (y, -x)
}

fn distance(instructions: &Vec<Step>) -> i64 {
    let mut facing = Direction::East;
    let mut x: i64 = 0;
    let mut y: i64 = 0;

    for i in instructions {
        // println!("({}, {}) {:?} : {:?} {}", x, y, facing, i.d, i.m);
        match i.d {
            Direction::North => {
                y -= i.m;
            }
            Direction::South => {
                y += i.m;
            }
            Direction::East => {
                x += i.m;
            }
            Direction::West => {
                x -= i.m;
            }

            Direction::Forward => {
                x += match facing {
                    Direction::East => i.m,
                    Direction::West => -i.m,
                    _ => 0,
                };
                y += match facing {
                    Direction::North => -i.m,
                    Direction::South => i.m,
                    _ => 0,
                };
            }

            Direction::Left | Direction::Right => {
                for _ in 0..(i.m / 90) {
                    facing = if i.d == Direction::Left {
                        turn_left(&facing)
                    } else {
                        turn_right(&facing)
                    };
                }
            }
        }
    }

    x.abs() + y.abs()
}

fn waypoint(instructions: &Vec<Step>) -> i64 {
    let mut wpx = 10;
    let mut wpy = 1;
    let mut sx = 0;
    let mut sy = 0;

    for i in instructions {
        // println!("({}, {}) & ({}, {})", sx, sy, wpx, wpy);
        match i.d {
            Direction::North => {
                wpy += i.m;
            }
            Direction::South => {
                wpy -= i.m;
            }
            Direction::East => {
                wpx += i.m;
            }
            Direction::West => {
                wpx -= i.m;
            }

            Direction::Forward => {
                sx += i.m * wpx;
                sy += i.m * wpy;
            }

            Direction::Left | Direction::Right => {
                for _ in 0..(i.m / 90) {
                    let (nx, ny) = if i.d == Direction::Left {
                        rotate_left(wpx, wpy)
                    } else {
                        rotate_right(wpx, wpy)
                    };
                    wpx = nx;
                    wpy = ny;
                }
            }
        }
    }

    sx.abs() + sy.abs()
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_data() -> Vec<String> {
        vec![
            "F10".to_string(),
            "N3".to_string(),
            "F7".to_string(),
            "R90".to_string(),
            "F11".to_string(),
        ]
    }

    #[test]
    fn test1() {
        let v = parse(&test_data());
        assert_eq!(distance(&v), 25);
    }

    #[test]
    fn test2() {
        let v = parse(&test_data());
        assert_eq!(waypoint(&v), 286);
    }
}
