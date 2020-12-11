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

fn checklist_part1(loc: usize, width: usize, max: usize) -> Vec<usize> {
    let mut surround = Vec::new();
    if loc >= width {
        surround.push(loc - width);
    }
    if loc % width > 0 {
        surround.push(loc - 1);
        if loc + width - 1 < max {
            surround.push(loc + width - 1);
        }
        if loc >= (width + 1) {
            surround.push(loc - width - 1);
        }
    }
    if (loc + 1) % width > 0 {
        surround.push(loc + 1);
        if loc + width + 1 < max {
            surround.push(loc + width + 1);
        }
        if loc > (width - 1) {
            surround.push(loc - width + 1);
        }
    }
    if loc + width < max {
        surround.push(loc + width);
    }

    surround
}

fn checklist_part2(loc: usize, layout: &Layout) -> Vec<usize> {
    let mut visible = Vec::new();

    // up
    let mut i = loc;
    while i >= layout.width {
        i -= layout.width;
        if *layout.seats.get(i).unwrap() != Position::Floor {
            visible.push(i);
            break;
        }
    }

    // down
    i = loc + layout.width;
    while i < layout.seats.len() {
        if *layout.seats.get(i).unwrap() != Position::Floor {
            visible.push(i);
            break;
        }
        i += layout.width;
    }

    // left
    i = loc;
    while i % layout.width > 0 {
        i -= 1;
        if *layout.seats.get(i).unwrap() != Position::Floor {
            visible.push(i);
            break;
        }
    }

    // right
    i = loc + 1;
    while i % layout.width > 0 && i < layout.seats.len() {
        if *layout.seats.get(i).unwrap() != Position::Floor {
            visible.push(i);
            break;
        }
        i += 1;
    }

    // up + right
    i = loc;
    while i >= layout.width {
        i -= layout.width - 1;
        if i % layout.width != 0 {
            if *layout.seats.get(i).unwrap() != Position::Floor {
                visible.push(i);
                break;
            }
        } else {
            break;
        }
    }

    // down + right
    i = loc + layout.width + 1;
    while i < layout.seats.len() {
        if i % layout.width != 0 {
            if *layout.seats.get(i).unwrap() != Position::Floor {
                visible.push(i);
                break;
            }
            i += layout.width + 1;
        } else {
            break;
        }
    }

    // up + left
    i = loc;
    while i >= layout.width {
        if i % layout.width != 0 {
            i -= layout.width + 1;
            if *layout.seats.get(i).unwrap() != Position::Floor {
                visible.push(i);
                break;
            }
        } else {
            break;
        }
    }

    // down + left
    i = loc + layout.width;
    while i < layout.seats.len() {
        if i % layout.width == 0 {
            break;
        }
        if *layout.seats.get(i - 1).unwrap() != Position::Floor {
            visible.push(i - 1);
            break;
        }
        i += layout.width - 1;
    }

    visible
}

fn step(part: i64, l: &Layout) -> (bool, Layout) {
    let mut next_l = Layout {
        width: l.width,
        rows: l.rows,
        seats: Vec::new(),
    };
    let mut flipped = false;
    let occ = if part == 1 { 4 } else { 5 };

    for (i, p) in l.seats.iter().enumerate() {
        if *p == Position::Floor {
            next_l.seats.push(Position::Floor);
            continue;
        }

        let surround = if part == 1 {
            checklist_part1(i, l.width, l.seats.len())
        } else {
            checklist_part2(i, &l)
        };

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
                    if count >= occ {
                        break;
                    }
                }
            }
            if count >= occ {
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
