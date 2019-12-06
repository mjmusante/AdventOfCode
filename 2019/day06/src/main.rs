use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use std::time::Instant;

use std::collections::HashMap;

struct Orbital {
    com: String, // center of mass
    sat: String, // orbiting satellite
}

fn main() {
    let f = File::open("inputs/day06.txt").unwrap();
    let vlist = BufReader::new(&f)
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    // let vlist = [
    //     "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L",
    // ];

    // let vlist = [
    //     "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L", "K)YOU",
    //     "I)SAN",
    // ];

    let now = Instant::now();

    let mut orbits = Vec::new();
    for v in &vlist {
        let orbital = &v.split(")").collect::<Vec<&str>>();
        orbits.push(Orbital {
            com: orbital[0].to_string(),
            sat: orbital[1].to_string(),
        });
    }

    let mut comlist = HashMap::new();
    for o in &orbits {
        comlist.insert(&o.sat, &o.com);
    }

    let mut part1 = 0;
    for v in comlist.keys() {
        let mut m = v;
        while comlist.contains_key(m) {
            part1 += 1;
            m = comlist.get(m).unwrap();
        }
    }


    // count steps back to origin
    let mut you_depth = 0;
    let mut you = "YOU".to_string();
    let mut steps = HashMap::new();
    while comlist.contains_key(&you) {
        steps.insert(you.clone(), you_depth);
        you_depth += 1;
        you = comlist.get(&you).unwrap().to_string();
    }

    // count steps back to first common ancestor
    let mut san = "SAN".to_string();
    let mut san_depth = 0;
    while comlist.contains_key(&san) {
        if steps.contains_key(&san) {
            break;
        }
        san_depth += 1;
        san = comlist.get(&san).unwrap().to_string();
    }

    // this node has the number of steps from YOU to the
    // common ancestor but includes the initial extra step
    let part2 = (steps.get(&san).unwrap() - 1) + (san_depth - 1);

    let exec_ms = now.elapsed().as_millis();

    println!("part 1 = {}", part1);
    println!("part 2 = {}", part2);
    println!("elapsed time = {}ms", exec_ms);
}
