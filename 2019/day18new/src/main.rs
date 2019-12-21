use std::process::exit;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use std::cmp::max;

use std::collections::{HashMap, HashSet};

type Pos = (i64, i64);
type Map = HashMap<Pos, char>;
type Key = HashSet<char>;
type RetType = (Pos, i64, char);

struct Maze {
    layout: Map,
    keys: Map,
    doors: Map,
    curloc: Pos,
    size: Pos,
}

fn main() {
    let f = File::open("inputs/day18.txt").unwrap();
    let vlist = BufReader::new(&f)
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    let maze = parse_maze(&vlist);
    print!("keys ='");
    for (_, v) in maze.keys.clone() {
        print!("{}", v);
    }
    println!("'");
    // dsearch(&maze);
    d4search(&maze);
}

/*
1  function Dijkstra(Graph, source):
2      dist[source] ← 0                           // Initialization
3
4      create vertex priority queue Q
5
6      for each vertex v in Graph:
7          if v ≠ source
8              dist[v] ← INFINITY                 // Unknown distance from source to v
9          prev[v] ← UNDEFINED                    // Predecessor of v
10
11         Q.add_with_priority(v, dist[v])
12
13
14     while Q is not empty:                      // The main loop
15         u ← Q.extract_min()                    // Remove and return best vertex
16         for each neighbor v of u:              // only v that are still in Q
17             alt ← dist[u] + length(u, v)
18             if alt < dist[v]
19                 dist[v] ← alt
20                 prev[v] ← u
21                 Q.decrease_priority(v, alt)
22
23     return dist, prev
---
1  S ← empty sequence
2  u ← target
3  if prev[u] is defined or u = source:          // Do something only if the vertex is reachable
4      while u is defined:                       // Construct the shortest path with a stack S
5          insert u at the beginning of S        // Push the vertex onto the stack
6          u ← prev[u]                           // Traverse from target to source
*/

type Index = (Pos, String);
type Value = (Index, i64);

fn hs2string(hs: &HashSet<char>) -> String {
    let mut m: Vec<char> = hs.iter().map(|ch| *ch).collect();
    m.sort_by(|a, b| a.cmp(b));
    m.into_iter().collect()
}

fn string2hs(s: &String) -> HashSet<char> {
    let mut ret = HashSet::new();
    for ch in s.chars() {
        ret.insert(ch);
    }
    ret
}

fn dsearch(maze: &Maze) {
    let mut dist: HashMap<Index, i64> = HashMap::new();
    let mut prev: HashMap<Index, Index> = HashMap::new();
    let mut queue: Vec<Value> = Vec::new();

    let origin: Index = (maze.curloc, String::new());
    dist.insert(origin.clone(), 0);
    queue.push((origin.clone(), 0));
    for s in find_current_keys(&maze, origin.0, &string2hs(&origin.1)) {
        let u: Index = (s.0, format!("{}", s.2));
        dist.insert(u.clone(), 999_999);
        queue.push((u, 999_999));
    }

    while queue.len() > 0 {
        let mut best = 0;
        for i in 1..queue.len() {
            if dist[&queue[i].0] < dist[&queue[best].0] {
                best = i;
            }
        }
        let u = queue.remove(best);
        println!("best - {:?}", u);
        for s in find_current_keys(&maze, (u.0).0, &string2hs(&(u.0).1)) {
            println!(" >>> {:?}", s);
            let mut hs = string2hs(&(u.0).1);
            hs.insert(s.2);
            let v: Index = (s.0, hs2string(&hs));
            let alt = dist[&u.0] + s.1;
            if !dist.contains_key(&v) || alt < dist[&v] {
                dist.insert(v.clone(), alt);
                prev.insert(v.clone(), u.0.clone());
                queue.push((v, alt));
            }
        }
    }
    // println!("DIST: {:?}", dist);
    // println!("PREV: {:?}", prev);
}

fn d4search(maze2: &Maze) {
    let mut layout = maze2.layout.clone();
    layout.remove(&(maze2.curloc.0 - 1, maze2.curloc.1));
    layout.remove(&(maze2.curloc.0 + 1, maze2.curloc.1));
    layout.remove(&(maze2.curloc.0, maze2.curloc.1 - 1));
    layout.remove(&(maze2.curloc.0, maze2.curloc.1 + 1));
    let maze = Maze {
        layout: layout,
        keys: maze2.keys.clone(),
        doors: maze2.doors.clone(),
        curloc: maze2.curloc,
        size: maze2.size,
    };

    let mut dist: HashMap<Index, i64> = HashMap::new();
    let mut prev: HashMap<Index, Index> = HashMap::new();
    let mut queue: Vec<Value> = Vec::new();

    let origin1: Index = ((maze.curloc.0 - 1, maze.curloc.1 - 1), String::new());
    let origin2: Index = ((maze.curloc.0 - 1, maze.curloc.1 + 1), String::new());
    let origin3: Index = ((maze.curloc.0 + 1, maze.curloc.1 - 1), String::new());
    let origin4: Index = ((maze.curloc.0 + 1, maze.curloc.1 + 1), String::new());

    dist.insert(origin1.clone(), 0);
    dist.insert(origin2.clone(), 0);
    dist.insert(origin3.clone(), 0);
    dist.insert(origin4.clone(), 0);
    queue.push((origin1.clone(), 0));
    queue.push((origin2.clone(), 0));
    queue.push((origin3.clone(), 0));
    queue.push((origin4.clone(), 0));
    for o in [origin1, origin2, origin3, origin4].iter() {
        for s in find_current_keys(&maze, o.0, &string2hs(&o.1)) {
            let u: Index = (s.0, format!("{}", s.2));
            dist.insert(u.clone(), 999_999);
            queue.push((u, 999_999));
        }
    }

    while queue.len() > 0 {
        let mut best = 0;
        for i in 1..queue.len() {
            if dist[&queue[i].0] < dist[&queue[best].0] {
                best = i;
            }
        }
        let u = queue.remove(best);
        println!("b4est - {:?}", u);
        for s in find_current_keys(&maze, (u.0).0, &string2hs(&(u.0).1)) {
            println!(" >>> {:?}", s);
            let mut hs = string2hs(&(u.0).1);
            hs.insert(s.2);
            let v: Index = (s.0, hs2string(&hs));
            let alt = dist[&u.0] + s.1;
            if !dist.contains_key(&v) || alt < dist[&v] {
                dist.insert(v.clone(), alt);
                prev.insert(v.clone(), u.0.clone());
                queue.push((v, alt));
            }
        }
    }
    // println!("DIST: {:?}", dist);
    // println!("PREV: {:?}", prev);
}

fn queue_search(maze: &Maze) {
    let mut dist: HashMap<Pos, i64> = HashMap::new();
    let mut prev: HashMap<Pos, RetType> = HashMap::new();
    let mut queue: Vec<RetType> = vec![];

    dist.insert(maze.curloc, 0);
    // Pos, i64, char
    for s in find_current_keys(&maze, maze.curloc, &HashSet::new()) {
        queue.push(s);
        dist.insert(s.0, s.1);
    }

    let mut keyset = HashSet::new();
    let mut last_u = (maze.curloc, 0, '0');
    while queue.len() > 0 {
        let mut index = 0;
        for i in 1..queue.len() {
            if queue[index].1 < queue[i].1 {
                index = i;
            }
        }
        let u = queue.remove(index);
        keyset.insert(u.2);
        for s in find_current_keys(&maze, u.0, &keyset) {
            let alt = dist.get(&u.0).unwrap() + s.1;
            println!(">> {:?} + dist {}", s, alt);
            if !dist.contains_key(&s.0) || alt < *dist.get(&s.0).unwrap() {
                dist.insert(s.0, alt);
                last_u = u.clone();
                prev.insert(s.0, u);
                queue.push((s.0, alt, s.2));
            }
        }
    }
    println!("DIST {:?}", dist);
    println!("PREV {:?}", prev);
    let mut S = vec![];
    let mut u = last_u;
    loop {
        S.push(u);
        if prev.contains_key(&u.0) {
            u = *prev.get(&u.0).unwrap();
        } else {
            break;
        }
    }
    println!("S: {:?}", S);
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
        keys: keys,
        doors: doors,
        // have_keys: HashSet::new(),
        curloc: start,
        size: (max_x, ypos),
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

fn find_current_keys(maze: &Maze, loc: Pos, keylist: &Key) -> Vec<RetType> {
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
