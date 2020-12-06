use std::collections::HashSet;

use aoc::utils::records;

pub fn run() {
    let groups = records("data/06.txt");

    // let groups_test = vec![ "abc", "a\nb\nc", "ab\nac", "a\na\na\na", "b"];
    // let groups_test = vec!["nlczsygmdabuorweqjhxfitv\nnmuvojghteyaxibwldsqrzfc\nmihetjnswbyzdvufcogxaqrl\niulxgqfoctnjhvrawbzemsdy\nexugdvclmqfzsojiwnbarhty\n"];

    let mut p1total = 0;
    let mut p2total = 0;
    for g in groups.iter() {
        let mut hs = HashSet::<char>::new();
        let mut inter;
        let mut cur = HashSet::<char>::new();
        let mut first = true;

        for c in g.chars() {
            match c {
                'a'..='z' => { hs.insert(c); cur.insert(c); },
                '\n' => {
                    if first {
                        inter = hs.clone();
                        first = false;
                    } else {
                        inter = inter.into_iter().filter(|e| cur.contains(e)).collect();
                    }
                    cur = HashSet::<char>::new();
                }
                c => { panic!("Unknown char {}", c); },
            }
        }
        if !cur.is_empty() {
            if first {
                inter = hs.clone();
            } else {
                inter = inter.into_iter().filter(|e| cur.contains(e)).collect();
            }
        }

        p1total += hs.len();
        p2total += inter.len();
    }

    println!("Part 1 = {}", p1total);
    println!("Part 2 = {}", p2total);
}
