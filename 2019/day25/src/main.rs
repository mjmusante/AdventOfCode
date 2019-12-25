use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use std::io;

use intcode::Computer;

fn main() {
    let mut c = Computer::new().from_file("inputs/day25.txt");
    let f = File::open("inputs/solve-day25.txt").unwrap();
    let mut commands = BufReader::new(&f)
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    println!("===============================================================================");

    c.run();
    while !c.halted() {
        while c.has_output() {
            print!("{}", c.next_output() as u8 as char);
        }

        if c.waiting_for_input() {
            let mut s;
            if commands.len() > 0 {
                s = String::from(commands.remove(0)) + "\n";
                while commands.len() > 0 && s.chars().nth(0).unwrap() == '#' {
                    s = String::from(commands.remove(0)) + "\n";
                }
            } else {
                s = String::new();
                io::stdin().read_line(&mut s).unwrap();
            }
            println!("> {}", s.to_uppercase());
            for i in s.chars() {
                c.run_with_input(i as i64);
            }
        }
    }
    while c.has_output() {
        print!("{}", c.next_output() as u8 as char);
    }
}
