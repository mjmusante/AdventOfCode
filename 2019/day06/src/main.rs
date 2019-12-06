use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

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

    /*
    let vlist = [
        "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L",
    ];
    */
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

    println!("part1={}", part1);
}
