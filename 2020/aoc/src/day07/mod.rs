use std::collections::HashSet;

use aoc::utils::lines;

pub fn run() {
    let lines = lines("data/07.txt");

    let bags = parse(&lines);
    let part1 = count(&bags);

    println!("Part 1 = {}", part1);
}

#[derive(Debug, Clone)]
struct Bag {
    name: String,
    containment: HashSet<String>,
}

fn parse(data: &Vec<String>) -> Vec<Bag> {
    let mut bags : Vec<Bag> = Vec::new();

    for d in data {
        let sp = d.find(" bags contain ").expect("invalid instruction: missing 'contain'");
        let (bag_name, remainder) = d.split_at(sp);
        let (_, contents) = remainder.split_at(14);
        if contents.starts_with("no ") {
            let b = Bag { name: bag_name.to_string(), containment: HashSet::new() };
            bags.push(b);
        } else {
            let mut hs = HashSet::new();
            for btype in contents.split(",") {
                let desc : Vec<&str> = btype.split_whitespace().collect();

                hs.insert(format!("{} {}", desc.get(1).unwrap(), desc.get(2).unwrap()));
            }
            bags.push(Bag { name: bag_name.to_string(), containment: hs } );
        }
    }

    bags
}

fn count(bags: &Vec<Bag>) -> usize {
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
        let bags = parse(&joe);
        assert_eq!(count(&bags), 4);
    }
}
