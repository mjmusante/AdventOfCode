use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use std::collections::{HashMap, HashSet};

use std::cmp::{max, min};

type Pos = (i64, i64);
type Pos3d = (i64, i64, i64);

fn get_vec() -> HashMap<Pos, Vec<Pos3d>> {
    let mut ret = HashMap::new();
    let std = vec![(0, -1, 0), (0, 1, 0), (0, 0, -1), (0, 0, 1)];

    // Tile A: 8, B, F, 12
    ret.insert((0, 0), vec![(-1, 2, 1), (0, 1, 0), (0, 0, 1), (-1, 1, 2)]);

    // Tile B: 8, A, C, G
    ret.insert((1, 0), vec![(-1, 1, 1), (0, -1, 0), (0, 1, 0), (0, 0, 1)]);

    // Tile C: 8, B, D, H
    ret.insert((2, 0), vec![(-1, 0, 1), (0, -1, 0), (0, 1, 0), (0, 0, 1)]);

    // Tile D: 8, C, E, I
    ret.insert((3, 0), vec![(-1, -1, 1), (0, -1, 0), (0, 1, 0), (0, 0, 1)]);

    // Tile E: 8, 14, D, J
    ret.insert(
        (4, 0),
        vec![(-1, -2, 1), (-1, -1, 2), (0, -1, 0), (0, 0, 1)],
    );

    // Tile F: 12, A, G, K
    ret.insert((0, 1), vec![(-1, 1, 1), (0, 0, -1), (0, 1, 0), (0, 0, 1)]);

    // Tile G: B, F, H, L
    ret.insert((1, 1), std.clone());

    // Tile H: G, C, I, 1, 2, 3, 4, 5
    ret.insert(
        (2, 1),
        vec![
            (0, -1, 0),
            (0, 0, -1),
            (0, 1, 0),
            (1, -2, -1),
            (1, -1, -1),
            (1, 0, -1),
            (1, 1, -1),
            (1, 2, -1),
        ],
    );

    // Tile I: D, H, J, N
    ret.insert((3, 1), std.clone());

    // Tile J: E, I, O, 14
    ret.insert((4, 1), vec![(0, 0, -1), (0, -1, 0), (0, 0, 1), (-1, -1, 1)]);

    // Tile K: F, L, P, 12
    ret.insert((0, 2), vec![(0, 0, -1), (0, 1, 0), (0, 0, 1), (-1, 1, 0)]);

    // Tile L: G, K, Q, 1, 6, 11, 16, 21
    ret.insert(
        (1, 2),
        vec![
            (0, 0, -1),
            (0, -1, 0),
            (0, 0, 1),
            (1, -1, -2),
            (1, -1, -1),
            (1, -1, 0),
            (1, -1, 1),
            (1, -1, 2),
        ],
    );

    // Tile N: I, O, S, 5, 10, 15, 20, 25
    ret.insert(
        (3, 2),
        vec![
            (0, 0, -1),
            (0, 1, 0),
            (0, 0, 1),
            (1, 1, -2),
            (1, 1, -1),
            (1, 1, 0),
            (1, 1, 1),
            (1, 1, 2),
        ],
    );

    // Tile O: J, N, T, 14
    ret.insert((4, 2), vec![(0, 0, -1), (0, -1, 0), (0, 0, 1), (-1, -1, 0)]);

    // Tile P: K, Q, U, 12
    ret.insert((0, 3), vec![(0, 0, -1), (0, 1, 0), (0, 0, 1), (-1, 1, -1)]);

    // Tile Q: L, P, R, V
    ret.insert((1, 3), std.clone());

    // Tile R: Q, S, W, 21, 22, 23, 24, 25
    ret.insert(
        (2, 3),
        vec![
            (0, -1, 0),
            (0, 0, 1),
            (0, 1, 0),
            (1, -2, 1),
            (1, -1, 1),
            (1, 0, 1),
            (1, 1, 1),
            (1, 2, 1),
        ],
    );

    // Tile S: N, R, T, X
    ret.insert((3, 3), std.clone());

    // Tile T: O, S, Y, 14
    ret.insert(
        (4, 3),
        vec![(0, 0, -1), (0, -1, 0), (0, 0, 1), (-1, -1, -1)],
    );

    // Tile U: P, V, 12, 18
    ret.insert(
        (0, 4),
        vec![(0, 0, -1), (0, 1, 0), (-1, 1, -2), (-1, 2, -1)],
    );

    // Tile V: Q, U, W, 18
    ret.insert((1, 4), vec![(0, 0, -1), (0, -1, 0), (0, 1, 0), (-1, 1, -1)]);

    // Tile W: R, V, X, 18
    ret.insert((2, 4), vec![(0, 0, -1), (0, -1, 0), (0, 1, 0), (-1, 0, -1)]);

    // Tile X: S, W, Y, 18
    ret.insert(
        (3, 4),
        vec![(0, 0, -1), (0, -1, 0), (0, 1, 0), (-1, -1, -1)],
    );

    // Tile Y: T, X, 14, 18
    ret.insert(
        (4, 4),
        vec![(0, 0, -1), (0, -1, 0), (-1, -1, -2), (-1, -2, -1)],
    );

    ret
}

fn tick3d(cur: &HashSet<Pos3d>, work: &HashMap<Pos, Vec<Pos3d>>) -> HashSet<Pos3d> {
    let mut ret = HashSet::new();
    let mut lowest = 0;
    let mut highest = 0;

    for c in cur {
        lowest = min(lowest, c.0);
        highest = max(highest, c.0);

        let index = (c.1, c.2);
        let mut count = 0;
        for i in &work[&index] {
            let lookup = (c.0 + i.0, c.1 + i.1, c.2 + i.2);
            if cur.contains(&lookup) {
                count += 1;
                if count > 1 {
                    break;
                }
            }
        }

        if count == 1 {
            ret.insert(c.clone());
        }
    }

    for d in (lowest - 1)..=(highest + 1) {
        for y in 0..5 {
            for x in 0..5 {
                if x == 2 && y == 2 {
                    continue;
                }
                if !cur.contains(&(d, x, y)) {
                    let index = (x, y);
                    let mut count = 0;
                    for i in &work[&index] {
                        let lookup = (d + i.0, x + i.1, y + i.2);
                        if cur.contains(&lookup) {
                            count += 1;
                            if count > 2 {
                                break;
                            }
                        }
                    }
                    if count == 1 || count == 2 {
                        ret.insert((d, x, y));
                    }
                }
            }
        }
    }

    ret
}

fn tick(cur: &HashSet<(i64, i64)>) -> HashSet<(i64, i64)> {
    let mut ret = HashSet::new();
    static SPIN: [(i64, i64); 4] = [(-1, 0), (0, -1), (0, 1), (1, 0)];

    for y in 0..5 {
        for x in 0..5 {
            let mut count = 0;
            for s in SPIN.iter() {
                if cur.contains(&(x + s.0, y + s.1)) {
                    count += 1;
                    if count > 2 {
                        break;
                    }
                }
            }
            if cur.contains(&(x, y)) {
                // bug survivies if there's 1 adjacent bug
                if count == 1 {
                    ret.insert((x, y));
                }
            } else {
                // bug spawns if there's 1 or 2 adjacent bugs
                if count == 1 || count == 2 {
                    ret.insert((x, y));
                }
            }
        }
    }
    ret
}

fn _show(eris: &HashSet<(i64, i64)>) {
    for y in 0..5 {
        for x in 0..5 {
            if eris.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
    println!("-");
}

fn score(eris: &HashSet<(i64, i64)>) -> i64 {
    let mut val = 1;
    let mut ret = 0;

    for y in 0..5 {
        for x in 0..5 {
            if eris.contains(&(x, y)) {
                ret += val;
            }
            val *= 2;
        }
    }

    ret
}

fn main() {
    let f = File::open("inputs/day24.txt").unwrap();
    let vlist = BufReader::new(&f)
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    let mut eris = HashSet::new();
    let mut eris3d = HashSet::new();
    for y in 0..vlist.len() {
        for x in 0..vlist[y].len() {
            if vlist[y].chars().nth(x) == Some('#') {
                eris.insert((x as i64, y as i64));
                eris3d.insert((0 as i64, x as i64, y as i64) as Pos3d);
            }
        }
    }

    let mut scoreset = HashSet::new();
    scoreset.insert(score(&eris));
    loop {
        eris = tick(&eris);
        let newscore = score(&eris);
        if scoreset.contains(&newscore) {
            println!("part 1 = {}", newscore);
            break;
        }
        scoreset.insert(newscore);
    }

    let work = get_vec();
    for _ in 0..200 {
        eris3d = tick3d(&eris3d, &work);
    }
    println!("part 2 = {}", eris3d.len());
}
