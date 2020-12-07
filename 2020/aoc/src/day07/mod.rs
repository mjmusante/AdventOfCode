use std::collections::{HashMap, HashSet};

use aoc::utils::lines;

pub fn run() {
    let lines = lines("data/07.txt");

    let (bags, data) = parse(&lines);
    let part1 = count_possible(&bags);
    let part2 = count_depth(&data, &"shiny gold".to_string());

    println!("Part 1 = {}", part1);
    println!("Part 2 = {}", part2);
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Inner {
    count: usize,
    name: String,
}

#[derive(Debug, Clone)]
struct Bag {
    name: String,
    containment: HashSet<String>,
}

type Data = HashMap<String, HashSet<Inner>>;

fn parse(data: &Vec<String>) -> (Vec<Bag>, Data) {
    let mut bags : Vec<Bag> = Vec::new();
    let mut forward : Data = HashMap::new();

    for d in data {
        let sp = d.find(" bags contain ").expect("invalid instruction: missing 'contain'");
        let (bag_name, remainder) = d.split_at(sp);
        let (_, contents) = remainder.split_at(14);
        if contents.starts_with("no ") {
            let b = Bag { name: bag_name.to_string(), containment: HashSet::new() };
            bags.push(b);
        } else {
            let mut hs = HashSet::new();
            let mut inner = HashSet::new();
            for btype in contents.split(",") {
                let desc_data : Vec<&str> = btype.split_whitespace().collect();
                let desc = format!("{} {}", desc_data.get(1).unwrap(), desc_data.get(2).unwrap());
                let count : usize = desc_data.get(0).unwrap().parse().unwrap();

                hs.insert(desc.clone());
                inner.insert(Inner { count, name: desc } );
            }
            bags.push(Bag { name: bag_name.to_string(), containment: hs } );
            forward.insert(bag_name.to_string(), inner);
        }
    }

    (bags, forward)
}

fn count_possible(bags: &Vec<Bag>) -> usize {
    let mut search = HashSet::new();
    let mut ignore = HashSet::new();

    for b in bags {
        if b.can_hold("shiny gold") {
            search.insert(b.name.clone());
            ignore.insert(b.name.clone());
        }
    }
    let mut count = search.len();

    while !search.is_empty() {
        let mut new_search = HashSet::new();

        for find in search {
            for b in bags {
                if ignore.contains(&b.name) {
                    continue;
                }
                if b.can_hold(&find) {
                    new_search.insert(b.name.clone());
                    ignore.insert(b.name.clone());
                }
            }
        }

        count += new_search.len();
        search = new_search;
    }

    count
}

fn count_depth(data: &Data, look_for: &String) -> usize {
    let mut count = 0;

    if data.contains_key(look_for) {
        for j in data.get(look_for).expect("could not find name in data") {
            count += j.count * (1 + count_depth(data, &j.name));
        }
    }

    count
}

impl Bag {
    pub fn can_hold(&self, desc: &str) -> bool {
        self.containment.contains(desc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try1() {
        let joe : Vec<String> = vec![
            "light red bags contain 1 bright white bag, 2 muted yellow bags.".to_string(),
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.".to_string(),
            "bright white bags contain 1 shiny gold bag.".to_string(),
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.".to_string(),
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.".to_string(),
            "dark olive bags contain 3 faded blue bags, 4 dotted black bags.".to_string(),
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.".to_string(),
            "faded blue bags contain no other bags.".to_string(),
            "dotted black bags contain no other bags.".to_string(),
            ];
        let (bags, data) = parse(&joe);
        assert_eq!(count_possible(&bags), 4);
        assert_eq!(count_depth(&data, &"shiny gold".to_string()), 32);
    }

    #[test]
    fn try2() {
        let joe : Vec<String> = vec![
            "shiny gold bags contain 2 dark red bags.".to_string(),
            "dark red bags contain 2 dark orange bags.".to_string(),
            "dark orange bags contain 2 dark yellow bags.".to_string(),
            "dark yellow bags contain 2 dark green bags.".to_string(),
            "dark green bags contain 2 dark blue bags.".to_string(),
            "dark blue bags contain 2 dark violet bags.".to_string(),
            "dark violet bags contain no other bags.".to_string(),
            ];
        let (_, data) = parse(&joe);
        assert_eq!(count_depth(&data, &"shiny gold".to_string()), 126);
    }
}
