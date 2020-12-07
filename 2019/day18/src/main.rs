use std::process::exit;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use std::cmp::max;

use std::collections::{HashMap, HashSet};

type Pos = (i64, i64);
type Map = HashMap<Pos, char>;
type Key = HashSet<char>;

struct Maze {
    layout: Map,
    keys: Map,
    doors: Map,
    curloc: Pos,
}

fn main() {
    let f = File::open("inputs/day18ex4.txt").unwrap();
    let vlist = BufReader::new(&f)
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    let maze = parse_maze(&vlist);
    unsafe {
        FOUNDLING = Some(HashMap::new());
    }

    // let part1 = find_accessible_keys(&maze, HashSet::new(), 0);

    let keylist = HashSet::new();
    println!(
        "part 1 = {}",
        breadth_first(&maze, maze.curloc, 0, &keylist)
    );
    // _show_maze(&maze);

    // println!("part 1 = {}", part1);
    println!("part 2 = {}", 2);
}

static mut FOUNDLING: Option<HashMap<(Pos, String), Vec<(Pos, i64, char)>>> = None;

fn breadth_first(maze: &Maze, loc: Pos, dist: i64, keys: &Key) -> i64 {
    if keys.len() == maze.keys.len() {
        return dist;
    }

    let mut blah2: Vec<char> = keys
        .iter()
        .map(|ch| *ch)
        .collect::<Vec<char>>()
        .into_iter()
        .collect();
    blah2.sort_by(|a, b| a.cmp(b));
    let blah3 = blah2.into_iter().collect::<String>();
    let blah: (Pos, String) = (loc, blah3.clone());
    let search;
    unsafe {
        match &FOUNDLING {
            Some(x) => {
                if x.contains_key(&blah) {
                    search = x.get(&blah).unwrap().clone();
                } else {
                    search = find_current_keys(&maze, loc, &keys);
                }
            }
            _ => {
                search = find_current_keys(&maze, loc, &keys);
            }
        }
    }

    let mut best = 0;
    // println!("starting from {:?}, distance {}, with keys {:?}", loc, dist, keys);
    // let search = find_current_keys(&maze, loc, &keys);
    for s in search.clone() {
        if best != 0 && best < s.1 {
            continue;
        }
        let mut nkl = keys.clone();
        nkl.insert(s.2);
        let b = breadth_first(&maze, s.0, dist + s.1, &nkl);
        if best == 0 || b < best {
            best = b;
            // println!("so far = {}", best);
        }
    }
    unsafe {
        match &FOUNDLING {
            Some(x) => {
                let mut y = x.clone();
                y.insert(blah, search);
                FOUNDLING = Some(y);
            }
            _ => {}
        }
    }

    best
}

fn parse_maze(text: &Vec<String>) -> Maze {
    let mut maze = HashMap::new();
    let mut keys = HashMap::new();
    let mut doors = HashMap::new();
    let mut xpos = 0;
    let mut ypos = 0;
    let mut max_x = 0;

    let mut start: Pos = (0, 0);

    for v in text {
        for ch in v.chars() {
            match ch {
                '#' => {}

                '.' | '@' => {
                    maze.insert((xpos, ypos), '.');
                    if ch == '@' {
                        start = (xpos, ypos);
                    }
                }

                'a'..='z' => {
                    maze.insert((xpos, ypos), '.');
                    keys.insert((xpos, ypos), ch);
                }

                'A'..='Z' => {
                    maze.insert((xpos, ypos), '.');
                    doors.insert((xpos, ypos), ch.to_lowercase().collect::<Vec<_>>()[0]);
                }

                _ => {
                    println!("what is a {}?", ch);
                    exit(1);
                }
            }
            xpos += 1;
        }
        max_x = max(xpos, max_x);
        xpos = 0;
        ypos += 1;
    }

    Maze {
        layout: maze,
        keys,
        doors,
        // have_keys: HashSet::new(),
        curloc: start,
    }
}

fn surrounding(maze: &Maze, loc: Pos, held: &Key) -> Vec<Pos> {
    let look = [
        (loc.0 - 1, loc.1),
        (loc.0 + 1, loc.1),
        (loc.0, loc.1 - 1),
        (loc.0, loc.1 + 1),
    ];
    let mut ret = vec![];

    for l in look.iter() {
        if maze.layout.contains_key(&l) {
            if maze.doors.contains_key(&l) {
                // do we have the key?
                let d = *maze.doors.get(&l).unwrap();
                if held.contains(&d) {
                    ret.push(*l);
                }
            } else {
                ret.push(*l);
            }
        }
    }

    ret
}

fn find_current_keys(maze: &Maze, loc: Pos, keylist: &Key) -> Vec<(Pos, i64, char)> {
    let mut found = vec![];
    let mut unvisited = vec![];
    let mut visited = HashSet::new();
    let mut newlist = keylist.clone();

    unvisited.push((loc, 0));
    while unvisited.len() > 0 {
        let m = unvisited.remove(0);
        // println!("\tvisiting {:?}", m);
        if maze.keys.contains_key(&m.0) {
            let k = *maze.keys.get(&m.0).unwrap();
            if !newlist.contains(&k) {
                visited.insert(m.0);
                found.push((m.0, m.1, k));
                newlist.insert(k);
                continue;
            }
        }
        for x in surrounding(maze, m.0, &keylist) {
            if !visited.contains(&x) {
                unvisited.push((x, m.1 + 1));
            }
        }
        visited.insert(m.0);
    }

    found
}
