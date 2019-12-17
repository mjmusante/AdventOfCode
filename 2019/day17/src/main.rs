use std::collections::HashMap;

use std::process::exit;

use intcode::Computer;

fn check_intersection(scaffold: &HashMap<(usize, usize), char>, x: usize, y: usize) -> char {
    let coords = vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];

    for c in coords {
        if scaffold.contains_key(&(c)) {
            if *scaffold.get(&(c)).unwrap() == '.' {
                return *scaffold.get(&(x, y)).unwrap();
            }
        }
    }
    'O'
}

fn main() {
    let mut c = Computer::new().from_file("inputs/day17.txt");
    let mut scaffold = HashMap::new();

    let mut xpos = 0;
    let mut ypos = 0;
    let mut max_x = 0;
    c.run();
    while c.has_output() {
        let ch = c.next_output() as u8 as char;
        match ch {
            '#' | '.' | 'v' | '^' | '<' | '>' => {
                scaffold.insert((xpos, ypos), ch);
                xpos += 1;
                if xpos > max_x {
                    max_x = xpos;
                }
            }
            '\n' => {
                xpos = 0;
                ypos += 1
            }
            _ => {
                println!("unexpected code {}", ch);
                exit(1);
            }
        }
    }
    if xpos > 0 {
        ypos += 1;
    }

    let mut part1 = 0;
    for y in 0..ypos {
        for x in 0..max_x {
            if scaffold.contains_key(&(x, y)) {
                let mut ch = *scaffold.get(&(x, y)).unwrap();
                if ch != '.' && x > 0 && y > 0 && x < (max_x - 1) {
                    ch = check_intersection(&scaffold, x, y);
                    if ch == 'O' {
                        part1 += x * y;
                    }
                }
                print!("{}", ch);
            }
        }
        println!("");
    }
    println!("part 1 = {}", part1);
}
