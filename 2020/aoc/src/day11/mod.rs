use aoc::utils::lines;

pub fn run() {
    let lines = lines("data/11.txt");
    let start = parse(&lines);

    let mut layout = step(&start);
    while layout.0 {
        layout = step(&layout.1);
    }
    println!("Part 1 = {}", occupied(&layout.1));
}

#[derive(PartialEq, Eq)]
enum Position {
    Floor,
    Empty,
    Occupied,
}

struct Layout {
    width: usize,
    rows: usize,
    seats: Vec<Position>,
}

fn parse(v: &Vec<String>) -> Layout {
    let mut l = Layout {
        width: v.get(0).expect("empty array").len(),
        rows: v.len(),
        seats: Vec::new(),
    };

    for row in v {
        for seat in row.chars() {
            match seat {
                'L' => l.seats.push(Position::Empty),
                '.' => l.seats.push(Position::Floor),
                ch => {
                    panic!(format!("Invalid char in input {}", ch));
                }
            }
        }
    }

    l
}

fn occupied(l: &Layout) -> usize {
    l.seats.iter().filter(|x| **x == Position::Occupied).count()
}

fn step(l: &Layout) -> (bool, Layout) {
    let mut next_l = Layout {
        width: l.width,
        rows: l.rows,
        seats: Vec::new(),
    };
    let mut flipped = false;

    for (i, p) in l.seats.iter().enumerate() {
        if *p == Position::Floor {
            next_l.seats.push(Position::Floor);
            continue;
        }

        let mut surround = Vec::new();
        if i >= l.width {
            surround.push(i - l.width);
        }
        if i % l.width > 0 {
            surround.push(i - 1);
            if i + l.width - 1 < l.seats.len() {
                surround.push(i + l.width - 1);
            }
            if i >= (l.width + 1) {
                surround.push(i - l.width - 1);
            }
        }
        if (i + 1) % l.width > 0 {
            surround.push(i + 1);
            if i + l.width + 1 < l.seats.len() {
                surround.push(i + l.width + 1);
            }
            if i > (l.width - 1) {
                surround.push(i - l.width + 1);
            }
        }
        if i + l.width < l.seats.len() {
            surround.push(i + l.width);
        }

        // print!("Position {}: [", i);
        // for x in &surround {
        //     print!(" {}", x);
        // }
        // println!(" ]");

        if *p == Position::Empty {
            let mut found = false;
            // if no occupied seats around us, then become occupied
            for j in surround {
                if j < l.seats.len() && *l.seats.get(j).unwrap() == Position::Occupied {
                    found = true;
                    break;
                }
            }
            if !found {
                flipped = true;
                next_l.seats.push(Position::Occupied);
            } else {
                next_l.seats.push(Position::Empty);
            }
        } else {
            let mut count = 0;
            for j in surround {
                if j < l.seats.len() && *l.seats.get(j).unwrap() == Position::Occupied {
                    count += 1;
                    if count >= 4 {
                        break;
                    }
                }
            }
            if count >= 4 {
                flipped = true;
                next_l.seats.push(Position::Empty);
            } else {
                next_l.seats.push(Position::Occupied);
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
        let (c1, n1) = step(&v);
        assert_eq!(c1, true);
        assert_eq!(occupied(&n1), 71);
        let (c2, n2) = step(&n1);
        assert_eq!(c2, true);
        assert_eq!(occupied(&n2), 20);
    }

    #[test]
    fn test2() {
        let v = parse(&test_data());
        let mut foo = step(&v);
        for _ in 0..4 {
            foo = step(&foo.1);
            assert!(foo.0);
        }
        foo = step(&foo.1);
        assert!(!foo.0);

    }
}
