use aoc::utils::lines;

pub fn run() {
    let file = lines("data/16.txt");
    let tickets = parse(&file);

    println!("Part 1 = {}", tickets.count_invalid());
}

#[derive(Debug)]
struct Range {
    min: i64,
    max: i64,
}

#[derive(Debug)]
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
        let mut result = Constraint { name, range: Vec::new() };
        for range in rangeinfo.split(" or ") {
            // println!("\trange = {}", range);
            let mut minmax = range.split('-');
            let min = minmax.next().unwrap().trim().parse::<i64>().expect("invalid range");
            let max = minmax.next().unwrap().trim().parse::<i64>().expect("invalid range");
            result.range.push(Range{ min, max });
        }

        result
    }
}

impl Ticket {
    pub fn invalid_by(&self, constraints: &Vec<Constraint>) -> i64 {
        let mut result = 0;

        for f in &self.fields {
            let mut found_valid = false;
            for c in constraints {
                for r in &c.range {
                    if *f >= r.min && *f <= r.max {
                        found_valid = true;
                        break;
                    }
                }
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
}

fn get_ticket(numstr: &String) -> Ticket {
    let mut fields = Vec::new();
    for n in numstr.split(',') {
        fields.push(n.parse::<i64>().expect("invalid number in ticket"));
    }

    Ticket{ fields }
}

fn parse(lines: &Vec<String>) -> Data {
    let mut result = Data { constraints: Vec::new(), my_ticket: Ticket { fields: Vec::new() }, nearby_tickets: Vec::new() };
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
}
