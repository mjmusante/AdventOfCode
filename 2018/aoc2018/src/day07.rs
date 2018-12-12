use lines;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq, Copy)]
struct DepGraphArrow {
    before: char,
    after: char
}

#[derive(Clone, Debug, Copy, PartialEq)]
struct Job {
    step: char,
    time_left: u64
}

fn part2(steps: &Vec<DepGraphArrow>, wcount: usize, offset: u8) -> String {
    let mut befores = HashSet::new();
    let mut afters = HashSet::new();
    let mut workers = HashMap::new();
    let mut sec = 0;
    let mut s1 = steps.clone();
    let mut last_one ='0';

    while s1.len() > 0 {

        befores.clear();
        afters.clear();
        for s in &s1 {
            befores.insert(s.before);
            afters.insert(s.after);
        }
        let mut jobs : Vec<char> = befores.difference(&afters).map(|x| *x).collect();

        jobs.sort();
        
        while workers.len() < wcount {
            if jobs.len() == 0 {
                break;
            }
            let j = jobs.remove(0);
            if workers.contains_key(&j) {
                continue;
            }
            workers.insert(j, Job{step: j, time_left: (j as u8 - 'A' as u8 + offset) as u64 + sec + 1,});
        }

        // do work
        sec += 1;

        // determine next work to do
        let mut s2 = vec![];
        let w2 = workers.clone();
        for s in &s1 {
            if workers.contains_key(&s.before) {
                let x = w2.get(&s.before).unwrap();
                if x.time_left != sec {
                    s2.push(s.clone());
                } else {
                    last_one = s.after;
                    workers.remove(&s.before);
                }
            } else if !w2.contains_key(&s.before) {
                s2.push(s.clone());
            }
        }
        s1 = s2;
    }
    (sec + 1 + (last_one as u8 - 'A' as u8 + offset) as u64).to_string()
}

fn parse(lines: &Vec<String>) -> Vec<DepGraphArrow> {
    let reg = Regex::new(r"^Step (\S) .*before step (\S)").unwrap();
    let mut order : Vec<DepGraphArrow> = vec![];

    for l in lines {
        let foo = reg.captures_iter(l).next().unwrap();
        let (b, a) = (foo[1].to_string(), foo[2].to_string());
        let (before, after) = (b.chars().next().unwrap(), a.chars().next().unwrap());

        order.push(DepGraphArrow { before: before, after: after });
    }

    order    
}

fn part1(inst: &Vec<DepGraphArrow>) -> String {
    let mut hm = HashMap::new();
    let mut order = inst.clone();

    for o in &order {
        *hm.entry(o.before).or_insert(0) += 1;
        *hm.entry(o.after).or_insert(0) += 1;
    }
        
    let mut ans = String::from("");

    while order.len() > 0 {
        for step in &order {
            hm.remove(&step.after);
        }
        let mut s = hm.keys().map(|k| *k).collect::<Vec<char>>();
        s.sort();
        ans.push(s[0]);
        // println!("{:?}", s);
        let mut order2 : Vec<DepGraphArrow> = vec![];
        let mut hm2 = HashMap::new();
        for step in &order {
            if s[0] != step.before {
                order2.push(*step);
                *hm2.entry(step.before).or_insert(0) += 1;
                *hm2.entry(step.after).or_insert(0) += 1;
            }
        }
        if order2.len() == 0 {
            // println!("{:?}", order[0].after);
            ans.push(order[0].after);
        }
        order = order2;
        hm = hm2;

    }

    ans
}

pub fn run() -> (String, String) {
    let lines = lines::lineread(String::from("puzzle_data/day07.txt"));
    let instructions = parse(&lines);

    (part1(&instructions), part2(&instructions, 5, 60))
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vec_of_strings {
        ($($x:expr),*) => (vec![$($x.to_string()),*]);
    }

    #[test]
    fn day07_test1() {
        let v = vec_of_strings![
            "Step C must be finished before step A can begin.",
            "Step C must be finished before step F can begin.",
            "Step A must be finished before step B can begin.",
            "Step A must be finished before step D can begin.",
            "Step B must be finished before step E can begin.",
            "Step D must be finished before step E can begin.",
            "Step F must be finished before step E can begin."
            ];
        assert_eq!(part1(&parse(&v)), "CABDFE".to_string());
    }

    #[test]
    fn day07_test2() {
        let v= vec_of_strings![
            "Step C must be finished before step A can begin.",
            "Step F must be finished before step E can begin.",
            "Step D must be finished before step E can begin.",
            "Step A must be finished before step B can begin.",
            "Step A must be finished before step D can begin.",
            "Step B must be finished before step E can begin.",
            "Step C must be finished before step F can begin."
        ];
        assert_eq!(part1(&parse(&v)), "CABDFE".to_string());
    }

    #[test]
    fn day07_test3() {
        let v= vec_of_strings![
            "Step B must be finished before step E can begin.",
            "Step C must be finished before step A can begin.",
            "Step F must be finished before step E can begin.",
            "Step D must be finished before step E can begin.",
            "Step A must be finished before step B can begin.",
            "Step A must be finished before step D can begin.",
            "Step C must be finished before step F can begin."
        ];
        assert_eq!(part1(&parse(&v)), "CABDFE".to_string());
    }

    #[test]
    fn day07_test4() {
        let v = vec_of_strings![
            "Step A must be finished before step B can begin.",
            "Step C must be finished before step D can begin.",
            "Step D must be finished before step E can begin.",
            "Step B must be finished before step E can begin.",
            "Step E must be finished before step G can begin.",
            "Step E must be finished before step F can begin.",
            "Step F must be finished before step H can begin.",
            "Step G must be finished before step H can begin."
        ];
        assert_eq!(part1(&parse(&v)), "ABCDEFGH".to_string());
    }

    #[test]
    fn day07_test5() {
        let mut order : Vec<DepGraphArrow> = vec![];
        order.push(DepGraphArrow { before: 'C', after: 'A' } );
        order.push(DepGraphArrow { before: 'C', after: 'F' } );
        order.push(DepGraphArrow { before: 'A', after: 'B' } );
        order.push(DepGraphArrow { before: 'A', after: 'D' } );
        order.push(DepGraphArrow { before: 'B', after: 'E' } );
        order.push(DepGraphArrow { before: 'D', after: 'E' } );
        order.push(DepGraphArrow { before: 'F', after: 'E' } );
        assert_eq!(part2(&order, 2, 0), "15".to_string());
    }
}
