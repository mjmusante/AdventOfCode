use std::collections::{HashMap, HashSet};

use aoc::utils::lines;

pub fn run() {
    let file = lines("data/16.txt");
    let tickets = parse(&file);

    println!("Part 1 = {}", tickets.count_invalid());
    println!("Part 2 = {}", tickets.departure());
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Range {
    min: i64,
    max: i64,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Constraint {
    name: String,
    range: Vec<Range>,
}

#[derive(Debug)]
struct Ticket {
    fields: Vec<i64>,
}

#[derive(Debug)]
struct Data {
    constraints: Vec<Constraint>,
    my_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

impl Constraint {
    pub fn new(name: String, rangeinfo: &String) -> Constraint {
        let mut result = Constraint {
            name,
            range: Vec::new(),
        };
        for range in rangeinfo.split(" or ") {
            // println!("\trange = {}", range);
            let mut minmax = range.split('-');
            let min = minmax
                .next()
                .unwrap()
                .trim()
                .parse::<i64>()
                .expect("invalid range");
            let max = minmax
                .next()
                .unwrap()
                .trim()
                .parse::<i64>()
                .expect("invalid range");
            result.range.push(Range { min, max });
        }

        result
    }

    pub fn valid(&self, num: i64) -> bool {
        for r in &self.range {
            if num >= r.min && num <= r.max {
                return true;
            }
        }
        false
    }
}

impl Ticket {
    pub fn invalid_by(&self, constraints: &Vec<Constraint>) -> i64 {
        let mut result = 0;

        for f in &self.fields {
            let mut found_valid = false;
            for c in constraints {
                found_valid = c.valid(*f);
                if found_valid {
                    break;
                }
            }

            if !found_valid {
                result += f;
            }
        }

        result
    }
}

impl Data {
    pub fn count_invalid(&self) -> i64 {
        let mut sum = 0;
        for ticket in &self.nearby_tickets {
            sum += ticket.invalid_by(&self.constraints);
        }

        sum
    }

    pub fn departure(&self) -> i64 {
        let mut check = Vec::new();
        if self.my_ticket.invalid_by(&self.constraints) != 0 {
            panic!("this can never work");
        }
        check.push(&self.my_ticket);
        for t in &self.nearby_tickets {
            if t.invalid_by(&self.constraints) == 0 {
                check.push(&t);
            }
        }

        let size = check[0].fields.len();
        let mut hm = HashMap::<usize, &Constraint>::new();
        let mut hs = HashSet::<&Constraint>::new();
        let mut answer = 1;

        while hs.len() < size {
            for i in 0..size {
                let mut oklist = Vec::new();
                for j in &self.constraints {
                    if hs.contains(j) {
                        continue;
                    }
                    let mut ok = true;
                    for c in &check {
                        if !j.valid(c.fields[i]) {
                            ok = false;
                            break;
                        }
                    }
                    if ok {
                        oklist.push(j);
                    }
                }
                if oklist.len() == 1 {
                    if oklist[0].name.starts_with("departure ") {
                        answer *= self.my_ticket.fields[i];
                    }
                    hm.insert(i, oklist[0]);
                    hs.insert(oklist[0]);
                    break;
                }
            }
        }

        answer
    }
}

fn get_ticket(numstr: &String) -> Ticket {
    let mut fields = Vec::new();
    for n in numstr.split(',') {
        fields.push(n.parse::<i64>().expect("invalid number in ticket"));
    }

    Ticket { fields }
}

fn parse(lines: &Vec<String>) -> Data {
    let mut result = Data {
        constraints: Vec::new(),
        my_ticket: Ticket { fields: Vec::new() },
        nearby_tickets: Vec::new(),
    };
    let mut phase = 0;
    for l in lines {
        if l == "" {
            phase += 1;
            continue;
        }
        if phase == 0 {
            // Constraints
            let mut x = l.split(':');
            let name = x.next().unwrap().to_string();
            // println!("   name = {}", name);
            let list = x.next().unwrap().to_string();
            result.constraints.push(Constraint::new(name, &list));
        } else if phase == 1 {
            // My ticket
            if l.starts_with("your") {
                continue;
            }
            result.my_ticket = get_ticket(&l);
            phase += 1;
        } else {
            // nearby tickets
            if l.starts_with("nearby") {
                continue;
            }
            result.nearby_tickets.push(get_ticket(&l));
        }
    }

    // println!("{:?}", result);

    result
}

#[cfg(test)]
mod test {

    use super::*;

    fn test_data() -> Vec<String> {
        vec![
            "class: 1-3 or 5-7".to_string(),
            "row: 6-11 or 33-44".to_string(),
            "seat: 13-40 or 45-50".to_string(),
            "".to_string(),
            "your ticket:".to_string(),
            "7,1,14".to_string(),
            "".to_string(),
            "nearby tickets:".to_string(),
            "7,3,47".to_string(),
            "40,4,50".to_string(),
            "55,2,20".to_string(),
            "38,6,12".to_string(),
        ]
    }

    #[test]
    fn test1() {
        let v = test_data();
        let tickets = parse(&v);
        assert_eq!(tickets.count_invalid(), 71);
    }

    fn test_data_2() -> Vec<String> {
        vec![
            "departure class: 0-1 or 4-19".to_string(),
            "departure row: 0-5 or 8-19".to_string(),
            "seat: 0-13 or 16-19".to_string(),
            "".to_string(),
            "your ticket:".to_string(),
            "11,12,13".to_string(),
            "".to_string(),
            "nearby tickets:".to_string(),
            "3,9,18".to_string(),
            "15,1,5".to_string(),
            "5,14,9".to_string(),
        ]
    }

    #[test]
    fn test2() {
        let v = test_data_2();
        let tickets = parse(&v);
        assert_eq!(tickets.departure(), 132);
    }
}
