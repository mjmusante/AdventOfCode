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
    let f = File::open("inputs/day18ex6.txt").unwrap();
    let vlist = BufReader::new(&f)
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();
    println!("{:?}", vlist);

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

fn show_map(maze: &Maze) {
    for y in 0..maze.size.1 {
        for x in 0..maze.size.0 {
            if maze.layout.contains_key(&(x, y)) {
                print!(".");
            } else {
                print!("#");
            }
        }
        println!("");
    }
}

fn d4search(maze2: &Maze) {
    show_map(&maze2);
    println!("-vs-");
    let mut layout = maze2.layout.clone();
    layout.remove(&(maze2.curloc.0 - 1, maze2.curloc.1));
    layout.remove(&(maze2.curloc.0 + 1, maze2.curloc.1));
    layout.remove(&(maze2.curloc.0, maze2.curloc.1 - 1));
    layout.remove(&(maze2.curloc.0, maze2.curloc.1 + 1));
    let maze = Maze {
        layout,
        keys: maze2.keys.clone(),
        doors: maze2.doors.clone(),
        curloc: maze2.curloc,
        size: maze2.size,
    };
    show_map(&maze);

    let mut dist: [HashMap<Index, i64>; 4] = [
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
    ];
    let mut queue: [Vec<Value>; 4] = [Vec::new(), Vec::new(), Vec::new(), Vec::new()];

    let origin1: Index = ((maze.curloc.0 - 1, maze.curloc.1 - 1), String::new());
    let origin2: Index = ((maze.curloc.0 - 1, maze.curloc.1 + 1), String::new());
    let origin3: Index = ((maze.curloc.0 + 1, maze.curloc.1 - 1), String::new());
    let origin4: Index = ((maze.curloc.0 + 1, maze.curloc.1 + 1), String::new());

    dist[0].insert(origin1.clone(), 0);
    dist[1].insert(origin2.clone(), 0);
    dist[2].insert(origin3.clone(), 0);
    dist[3].insert(origin4.clone(), 0);
    queue[0].push((origin1.clone(), 0));
    queue[1].push((origin2.clone(), 0));
    queue[2].push((origin3.clone(), 0));
    queue[3].push((origin4.clone(), 0));
    let mut i = 0;
    for o in [origin1, origin2, origin3, origin4].iter() {
        for s in find_current_keys(&maze, o.0, &string2hs(&o.1)) {
            let u: Index = (s.0, format!("{}", s.2));
            dist[i].insert(u.clone(), 999_999);
            queue[i].push((u, 999_999));
        }
        i += 1;
    }

    let mut q = 0;
    let mut pass = 0;
    while queue[0].len() > 0 || queue[1].len() > 0 || queue[2].len() > 0 || queue[3].len() > 0 {
        q = (q + 1) % 4;
        if queue[q].len() == 0 {
            q = (q + 1) % 4;
        }
        let mut best = 0;
        for i in 1..queue[q].len() {
            if dist[q][&queue[q][i].0] < dist[q][&queue[q][best].0] {
                best = i;
            }
        }
        let u = queue[q].remove(best);
        println!("b4est - {:?}", u);
        let m = find_current_keys(&maze, (u.0).0, &string2hs(&(u.0).1));
        if m.len() == 0 {
            // put it at the back of the queue in case we can pick up some other keys
            // in other quadrants
            queue[q].push(u);
            pass += 1;
        } else {
            pass = 0;
            for s in m {
                println!(" >>> {:?}", s);
                let mut hs = string2hs(&(u.0).1);
                hs.insert(s.2);
                let v: Index = (s.0, hs2string(&hs));
                let alt = dist[q][&u.0] + s.1;
                if !dist[q].contains_key(&v) || alt < dist[q][&v] {
                    dist[q].insert(v.clone(), alt);
                    queue[q].push((v, alt));
                }
            }
        }

        if pass == 4 {
            println!("no work for 4 passes - quitting");
            break;
        }
    }
    // println!("DIST: {:?}", dist);
    // println!("PREV: {:?}", prev);
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
    println!("({}, {})", max_x, ypos);

    Maze {
        layout: maze,
        keys,
        doors,
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
