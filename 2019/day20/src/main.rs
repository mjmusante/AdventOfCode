// use std::process::exit;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use std::cmp::max;

use std::collections::{HashMap, HashSet};

type Pos = (i64, i64);

struct Maze {
    layout: HashSet<Pos>,
    portals: HashMap<Pos, Pos>,
    start: Pos,
    end: Pos,
    bounds: Pos,
}

fn main() {
    let f = File::open("inputs/day20.txt").unwrap();
    let vlist = BufReader::new(&f)
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();
    let maze = parse_maze(&vlist);
    show_paths(&maze);
    show_depth_paths(&maze);
}

fn get_char(s: &String, i: i64) -> char {
    s.chars().nth(i as usize).unwrap()
}

fn parse_maze(lines: &Vec<String>) -> Maze {
    let mut ret = Maze {
        layout: HashSet::new(),
        portals: HashMap::new(),
        start: (0, 0),
        end: (0, 0),
        bounds: (0, 0),
    };
    let mut chars: HashMap<Pos, char> = HashMap::new();

    for uy in 0..lines.len() {
        for ux in 0..lines[uy].len() {
            let y = uy as i64;
            let x = ux as i64;
            let ch = get_char(&lines[uy], x);
            if ch == '.' {
                ret.layout.insert((x, y));
                ret.bounds.0 = max(ret.bounds.0, x);
                ret.bounds.1 = max(ret.bounds.1, y);
            } else if ch.is_alphabetic() {
                chars.insert((x, y), ch);
            }
        }
    }

    let mut plist: HashMap<String, Pos> = HashMap::new();
    for (k, v) in &chars {
        let foo;
        let bar;
        if chars.contains_key(&(k.0 + 1, k.1)) {
            bar = (k.0 + 1, k.1);
            if ret.layout.contains(&(k.0 - 1, k.1)) {
                foo = (k.0 - 1, k.1);
            } else {
                assert!(ret.layout.contains(&(k.0 + 2, k.1)));
                foo = (k.0 + 2, k.1);
            }
        } else if chars.contains_key(&(k.0, k.1 + 1)) {
            bar = (k.0, k.1 + 1);
            if ret.layout.contains(&(k.0, k.1 - 1)) {
                foo = (k.0, k.1 - 1);
            } else {
                assert!(ret.layout.contains(&(k.0, k.1 + 2)));
                foo = (k.0, k.1 + 2);
            }
        } else {
            continue;
        }
        let s = format!("{}{}", v, chars.get(&bar).unwrap());
        if plist.contains_key(&s) {
            let alt = *plist.get(&s).unwrap();
            ret.portals.insert(foo, alt);
            ret.portals.insert(alt, foo);
        } else if s == "AA" {
            ret.start = foo;
        } else if s == "ZZ" {
            ret.end = foo;
        } else {
            plist.insert(s, foo);
        }
    }

    ret
}

fn show_paths(maze: &Maze) {
    // println!("Starting at {:?}", maze.start);

    let mut unvisited = vec![];
    let mut visited = HashSet::new();

    unvisited.push((maze.start, 0));
    // println!("push ({:?}, {})", maze.start, 0);
    while unvisited.len() > 0 {
        let m = unvisited.remove(0);
        if m.0 == maze.end {
            println!("One way is {} steps", m.1);
        }
        for x in surrounding(&maze, m.0) {
            if !visited.contains(&x) {
                // println!("push ({:?}, {})", x, m.1 + 1);
                unvisited.push((x, m.1 + 1));
            }
        }
        visited.insert(m.0);
        // println!("Visited {:?}", m.0);
    }
}

fn surrounding(maze: &Maze, loc: Pos) -> Vec<Pos> {
    let look = [
        (loc.0 - 1, loc.1),
        (loc.0 + 1, loc.1),
        (loc.0, loc.1 - 1),
        (loc.0, loc.1 + 1),
    ];
    let mut ret = vec![];

    if maze.portals.contains_key(&loc) {
        ret.push(*maze.portals.get(&loc).unwrap());
    }

    for l in look.iter() {
        if maze.layout.contains(l) {
            ret.push(*l);
        }
    }

    ret
}

fn show_depth_paths(maze: &Maze) {
    let mut unvisited = vec![];
    let mut visited = HashSet::new();

    unvisited.push(((maze.start, 0), 0));
    while unvisited.len() > 0 {
        let m = unvisited.remove(0);
        // println!("Visiting {:?}", m);
        if (m.0).0 == maze.end && (m.0).1 == 0 {
            println!("Found path of {} steps", m.1);
        }
        for x in surrounding_depth(&maze, m.0) {
            if !visited.contains(&x) {
                unvisited.push((x, m.1 + 1))
            }
        }
        visited.insert(m.0);
    }
}

fn surrounding_depth(maze: &Maze, loc: (Pos, i64)) -> Vec<(Pos, i64)> {
    let look = [
        ((loc.0).0 - 1, (loc.0).1),
        ((loc.0).0 + 1, (loc.0).1),
        ((loc.0).0, (loc.0).1 - 1),
        ((loc.0).0, (loc.0).1 + 1),
    ];
    let mut ret = vec![];

    if maze.portals.contains_key(&loc.0) {
        let newloc = *maze.portals.get(&(loc.0)).unwrap();
        // going down or up?
        if (loc.0).0 == 2
            || (loc.0).1 == 2
            || (loc.0).0 == maze.bounds.0
            || (loc.0).1 == maze.bounds.1
        {
            // going up!
            if loc.1 > 0 {
                ret.push((newloc, loc.1 - 1));
            }
        } else if loc.1 < 40 {
            // going down!
            ret.push((newloc, loc.1 + 1));
        }
        // println!("depth {} from {:?} to {:?}", newdepth, loc.0, newloc);
    }
    for l in look.iter() {
        if maze.layout.contains(l) {
            ret.push((*l, loc.1));
        }
    }

    ret
}
