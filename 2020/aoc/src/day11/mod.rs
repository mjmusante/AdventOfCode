use std::collections::HashMap;

use aoc::utils::lines;

pub fn run() {
    let lines = lines("data/11.txt");
    let start = parse(&lines);

    let mut layout = step(1, &start);
    while layout.0 {
        layout = step(1, &layout.1);
    }
    println!("Part 1 = {}", occupied(&layout.1));

    layout = step(2, &parse(&lines));
    while layout.0 {
        layout = step(2, &layout.1);
    }
    println!("Part 2 = {}", occupied(&layout.1));
}

#[derive(Debug, PartialEq, Eq)]
enum Position {
    Floor,
    Empty,
    Occupied,
}

struct Layout {
    width: i64,
    height: i64,
    seats: HashMap<(i64, i64), Position>,
}

fn parse(v: &Vec<String>) -> Layout {
    let mut l = Layout {
        width: v.get(0).expect("empty array").len() as i64,
        height: v.len() as i64,
        seats: HashMap::new(),
    };

    let mut row = 0;
    for line in v {
        let mut col = 0;
        for seat in line.chars() {
            l.seats.insert(
                (row, col),
                match seat {
                    'L' => Position::Empty,
                    '.' => Position::Floor,
                    ch => {
                        panic!(format!("Invalid char in input {}", ch));
                    }
                },
            );
            col += 1;
        }
        row += 1;
    }

    l
}

fn occupied(l: &Layout) -> usize {
    l.seats
        .iter()
        .filter(|x| *x.1 == Position::Occupied)
        .count()
}

fn checklist_part1(loc: (i64, i64)) -> Vec<(i64, i64)> {
    vec![
        (loc.0 - 1, loc.1 - 1),
        (loc.0 - 1, loc.1),
        (loc.0 - 1, loc.1 + 1),
        (loc.0, loc.1 - 1),
        (loc.0, loc.1 + 1),
        (loc.0 + 1, loc.1 - 1),
        (loc.0 + 1, loc.1),
        (loc.0 + 1, loc.1 + 1),
    ]
}

fn checklist_part2(loc: (i64, i64), layout: &Layout) -> Vec<(i64, i64)> {
    let mut visible = Vec::new();

    let dirs = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    for d in dirs {
        let mut row = loc.0 + d.0;
        let mut col = loc.1 + d.1;
        while row >= 0 && row < layout.height && col >= 0 && col < layout.width {
            match layout.seats.get(&(row, col)) {
                Some(Position::Floor) => (),
                _ => {
                    visible.push((row, col));
                    break;
                }
            }
            row += d.0;
            col += d.1;
        }
    }

    visible
}

fn step(part: i64, l: &Layout) -> (bool, Layout) {
    let mut next_l = Layout {
        width: l.width,
        height: l.height,
        seats: HashMap::new(),
    };
    let mut flipped = false;
    let occ = if part == 1 { 4 } else { 5 };

    for row in 0..l.height {
        for col in 0..l.width {
            let p = l.seats.get(&(row, col)).unwrap();

            if *p == Position::Floor {
                next_l.seats.insert((row, col), Position::Floor);
                continue;
            }

            let surround = if part == 1 {
                checklist_part1((row, col))
            } else {
                checklist_part2((row, col), &l)
            };

            if *p == Position::Empty {
                let mut found = false;
                for j in surround {
                    if l.seats.get(&j) == Some(&Position::Occupied) {
                        found = true;
                        break;
                    }
                }
                if !found {
                    flipped = true;
                    next_l.seats.insert((row, col), Position::Occupied);
                } else {
                    next_l.seats.insert((row, col), Position::Empty);
                }
            } else {
                let mut count = 0;
                for j in surround {
                    if l.seats.get(&j) == Some(&Position::Occupied) {
                        count += 1;
                        if count >= occ {
                            break;
                        }
                    }
                }
                if count >= occ {
                    flipped = true;
                    next_l.seats.insert((row, col), Position::Empty);
                } else {
                    next_l.seats.insert((row, col), Position::Occupied);
                }
            }
        }
    }

    (flipped, next_l)
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_data() -> Vec<String> {
        vec![
            "L.LL.LL.LL".to_string(),
            "LLLLLLL.LL".to_string(),
            "L.L.L..L..".to_string(),
            "LLLL.LL.LL".to_string(),
            "L.LL.LL.LL".to_string(),
            "L.LLLLL.LL".to_string(),
            "..L.L.....".to_string(),
            "LLLLLLLLLL".to_string(),
            "L.LLLLLL.L".to_string(),
            "L.LLLLL.LL".to_string(),
        ]
    }

    #[test]
    fn test1() {
        let v = parse(&test_data());
        let (c1, n1) = step(1, &v);
        assert_eq!(c1, true);
        assert_eq!(occupied(&n1), 71);
        let (c2, n2) = step(1, &n1);
        assert_eq!(c2, true);
        assert_eq!(occupied(&n2), 20);
    }

    #[test]
    fn test2() {
        let v = parse(&test_data());
        let mut foo = step(1, &v);
        for _ in 0..4 {
            foo = step(1, &foo.1);
            assert!(foo.0);
        }
        foo = step(1, &foo.1);
        assert!(!foo.0);
    }

    #[test]
    fn test3() {
        let v = parse(&test_data());
        let mut layout = step(2, &v);
        assert!(layout.0);
        assert_eq!(occupied(&layout.1), 71);
        layout = step(2, &layout.1);
        assert!(layout.0);
        assert_eq!(occupied(&layout.1), 7);
        layout = step(2, &layout.1);
        assert!(layout.0);
        assert_eq!(occupied(&layout.1), 53);
        layout = step(2, &layout.1);
        assert!(layout.0);
        assert_eq!(occupied(&layout.1), 18);
        layout = step(2, &layout.1);
        assert!(layout.0);
        assert_eq!(occupied(&layout.1), 31);
        layout = step(2, &layout.1);
        assert!(layout.0);
        assert_eq!(occupied(&layout.1), 26);
        layout = step(2, &layout.1);
        assert!(!layout.0);
        assert_eq!(occupied(&layout.1), 26);
    }

    #[test]
    fn test4() {
        let v = parse(&test_data());
        let mut layout = step(2, &v);
        while layout.0 {
            layout = step(2, &layout.1);
        }
        assert_eq!(occupied(&layout.1), 26);
    }
}
