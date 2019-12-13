use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use std::collections::HashMap;

use intcode::Computer;

fn _show_map(map: &HashMap<(i64, i64), i64>) {
    for y in 0.. {
        let mut printed = 0;
        for x in 0..78 {
            if map.contains_key(&(x, y)) {
                let ch = match *map.get(&(x, y)).unwrap() {
                    0 => ' ',
                    1 => '#',
                    2 => '?',
                    3 => '_',
                    4 => 'o',
                    _ => 'X',
                };
                print!("{}", ch);
                printed += 1;
            } else {
                print!(".");
            }
        }
        println!("");
        if printed == 0 {
            break;
        }
    }
}

fn main() {
    let f = File::open("inputs/day13.txt").unwrap();
    let vlist = BufReader::new(&f)
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    let ary = vlist[0]
        .split(",")
        .map(|num| num.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let mut c = Computer::new(&ary);
    c.run();

    let mut part1 = 0;
    while c.has_output() {
        let _x = c.next_output();
        let _y = c.next_output();
        let g = c.next_output();
        if g == 2 {
            part1 += 1;
        }
    }

    println!("part 1 = {}", part1);

    let mut board = HashMap::new();
    let mut game = Computer::new(&ary);
    game.poke(0, 2);
    let mut paddlex = 0;

    while !game.halted() {
        let mut ballx = 0;
        game.run();
        while game.has_output() {
            let x = game.next_output();
            let y = game.next_output();
            let g = game.next_output();
            if x >= 0 {
                board.insert((x, y), g);
                if g == 4 {
                    ballx = x;
                } else if g == 3 {
                    paddlex = x;
                }
            }
        }
        game.run_with_input(if ballx < paddlex {
            -1
        } else if ballx > paddlex {
            1
        } else {
            0
        });
    }

    while game.has_output() {
        let x = game.next_output();
        let y = game.next_output();
        let g = game.next_output();
        if x == -1 && y == 0 {
            println!("part 2 = {}", g);
        }
    }
}
