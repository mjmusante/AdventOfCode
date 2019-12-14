use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use std::collections::HashMap;

#[derive(Debug)]
struct Reaction {
    amount: i64,
    needs: Vec<(i64, String)>,
}

fn need_for(
    map: &HashMap<String, Reaction>,
    leftover: &mut HashMap<String, i64>,
    amt: i64,
    resource: &String,
) -> HashMap<String, i64> {
    let r = map.get(resource).unwrap();
    let mut req = amt;

    if leftover.contains_key(resource) {
        let mut stake = *leftover.get(resource).unwrap();
        if stake == req {
            leftover.remove(resource);
            return HashMap::new();
        } else if stake > req {
            stake -= req;
            leftover.insert(resource.to_string(), stake);
            return HashMap::new();
        }
        req -= stake;
        leftover.remove(resource);
    }

    let scale = (req + r.amount - 1) / r.amount;
    let overflow = r.amount * scale - req;
    if overflow > 0 {
        leftover.insert(resource.to_string(), overflow);
    }

    let mut x = HashMap::new();
    for i in &r.needs {
        x.insert(i.1.clone(), i.0 * scale);
    }

    for (k, v) in x.clone() {
        if k != "ORE" {
            let sub = need_for(map, leftover, v, &k.clone());
            for (sk, sv) in sub {
                if x.contains_key(&sk) {
                    let nv = *x.get(&sk).unwrap() + sv;
                    x.insert(sk, nv);
                } else {
                    x.insert(sk, sv);
                }
            }
        }
    }
    x
}

fn for_n_ore(map: &HashMap<String, Reaction>, n: i64) -> i64 {
    let mut leftover = HashMap::new();
    let mut total_ore = 0;
    let xyzzy = need_for(&map, &mut leftover, n, &"FUEL".to_string());
    for (k, v) in xyzzy {
        if map.contains_key(&k) {
            let r = map.get(&k).unwrap();
            if r.needs[0].1 == "ORE" {
                let produce = v / r.amount + if v % r.amount == 0 { 0 } else { 1 };
                total_ore += r.needs[0].0 * produce;
            }
        }
    }

    total_ore
}

fn main() {
    // let example1 = vec![
    //     "10 ORE => 10 A".to_string(),
    //     "1 ORE => 1 B".to_string(),
    //     "7 A, 1 B => 1 C".to_string(),
    //     "7 A, 1 C => 1 D".to_string(),
    //     "7 A, 1 D => 1 E".to_string(),
    //     "7 A, 1 E => 1 FUEL".to_string(),
    //     ];

    // let example1 = vec![
    //     "9 ORE => 2 A",
    //     "8 ORE => 3 B",
    //     "7 ORE => 5 C",
    //     "3 A, 4 B => 1 AB",
    //     "5 B, 7 C => 1 BC",
    //     "4 C, 1 A => 1 CA",
    //     "2 AB, 3 BC, 4 CA => 1 FUEL",
    // ];

    // let example1 = vec![
    //     "157 ORE => 5 NZVS",
    //     "165 ORE => 6 DCFZ",
    //     "44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL",
    //     "12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ",
    //     "179 ORE => 7 PSHF",
    //     "177 ORE => 5 HKGWZ",
    //     "7 DCFZ, 7 PSHF => 2 XJWVT",
    //     "165 ORE => 2 GPVTF",
    //     "3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT",
    // ];

    // let example1 = vec![
    //     "5 ORE => 3 C",
    //     "13 ORE => 5 B",
    //     "7 C, 11 B => 3 A",
    //     "7 A, 1 B => 1 FUEL",
    // ];

    // let example1 = vec![
    //     "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG",
    //     "17 NVRVD, 3 JNWZP => 8 VPVL",
    //     "53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL",
    //     "22 VJHF, 37 MNCFX => 5 FWMGM",
    //     "139 ORE => 4 NVRVD",
    //     "144 ORE => 7 JNWZP",
    //     "5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC",
    //     "5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV",
    //     "145 ORE => 6 MNCFX",
    //     "1 NVRVD => 8 CXFTF",
    //     "1 VJHF, 6 MNCFX => 4 RFSQX",
    //     "176 ORE => 6 VJHF",
    // ];

    let f = File::open("inputs/day14.txt").unwrap();
    let vlist = BufReader::new(&f)
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    let mut map = HashMap::new();
    for e in &vlist {
        let produce: Vec<&str> = e.split("=>").into_iter().collect();

        let product: Vec<&str> = produce[1].trim().split(" ").into_iter().collect();
        let amount = product[0].parse::<i64>().unwrap();

        let mut reactants = vec![];
        let source: Vec<&str> = produce[0].split(",").into_iter().collect();
        for s in source {
            let div: Vec<&str> = s.trim().split(" ").into_iter().collect();
            reactants.push((div[0].parse::<i64>().unwrap(), div[1].trim().to_string()));
        }
        let r = Reaction {
            amount: amount,
            needs: reactants,
        };

        map.insert(product[1].to_string(), r);
    }

    println!("part 1 = {}", for_n_ore(&map, 1));
}
