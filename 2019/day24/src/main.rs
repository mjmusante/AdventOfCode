use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use std::collections::HashSet;

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
    for y in 0..vlist.len() {
        for x in 0..vlist[y].len() {
            if vlist[y].chars().nth(x) == Some('#') {
                eris.insert((x as i64, y as i64));
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
}
