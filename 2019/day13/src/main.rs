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
    let mut game = Computer::new().from_file("inputs/day13.txt");
    game.run();

    let mut part1 = 0;
    while game.has_output() {
        let _x = game.next_output();
        let _y = game.next_output();
        let g = game.next_output();
        if g == 2 {
            part1 += 1;
        }
    }

    println!("part 1 = {}", part1);

    let mut board = HashMap::new();
    game.reset();
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
        game.run_with_input((ballx - paddlex).signum());
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
