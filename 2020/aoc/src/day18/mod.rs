use aoc::utils::lines;

pub fn run() {
    let lines = lines("data/18.txt");
    let mut sum = 0;
    for l in &lines {
        sum += eval(l);
    }

    println!("Part 1 = {}", sum);
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Symbol {
    Num(i64),
    Add,
    Multiply,
    Paren
}

fn eval(expr: &String) -> i64 {
    let mut rpn = Vec::<Symbol>::new();
    let mut stack = Vec::<&Symbol>::new();
    let mut num : i64 = 0;
    let mut num_started = false;

    for c in expr.chars() {
        if !c.is_ascii_digit() && num_started {
            rpn.push(Symbol::Num(num));
            num = 0;
            num_started = false;
        }

        if c.is_ascii_whitespace() {
            continue;
        }
        if c.is_ascii_digit() {
            num_started = true;
            num = (10 * num) + c.to_digit(10).unwrap() as i64;
            continue;
        }
        if c == '+' || c == '*' {
            if let Some(sym) = stack.last() {
                if *sym == &Symbol::Add || *sym == &Symbol::Multiply {
                    rpn.push(*stack.pop().unwrap());
                }
            }
            stack.push(match c { '+' => &Symbol::Add, '*' => &Symbol::Multiply, _ => panic!("yikes") });
        } else if c == '(' {
            stack.push(&Symbol::Paren);
        } else if c == ')' {
            loop {
                if let Some(sym) = stack.pop() {
                    match sym {
                        Symbol::Add | Symbol::Multiply => { rpn.push(*sym); },
                        Symbol::Paren => { break; },
                        _ => panic!("number on stack"),
                    }
                }
            }
        } else {
            panic!(format!("Invalid char {} in expression", c));
        }
    }

    if num_started {
        rpn.push(Symbol::Num(num));
    }
    while !stack.is_empty() {
        rpn.push(*stack.pop().unwrap());
    }

    // println!("rpn = {:?}", rpn);

    let mut mem = Vec::<i64>::new();
    for op in &rpn {
        match op {
            Symbol::Num(n) => { mem.push(*n); },
            Symbol::Add => {
                let i = mem.pop().unwrap();
                let j = mem.pop().unwrap();
                // println!("Adding {} and {}", i, j);
                mem.push(i + j);
            },
            Symbol::Multiply => {
                let i = mem.pop().unwrap();
                let j = mem.pop().unwrap();
                // println!("Multiplying {} and {}", i, j);
                mem.push(i * j);
            }
            e => {
                panic!(format!("Invalid symbol on result {:?}", e));
            }
        }
    }

    assert_eq!(mem.len(), 1);
    mem.pop().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(eval(&"1 + 2 * 3 + 4 * 5 + 6".to_string()), 71);
    }

    #[test]
    fn test2() {
        assert_eq!(eval(&"2 * 3 + (4 * 5)".to_string()), 26);
    }

    #[test]
    fn test3() {
        assert_eq!(eval(&"5 + (8 * 3 + 9 + 3 * 4 * 3)".to_string()), 437);
    }

    #[test]
    fn test4() {
        assert_eq!(eval(&"5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".to_string()), 12240);
    }

    #[test]
    fn test5() {
        assert_eq!(eval(&"((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2".to_string()), 13632);
    }
}
