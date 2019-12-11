use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use std::collections::HashMap;

use intcode::Computer;

fn main() {
    let f = File::open("inputs/day11.txt").unwrap();
    let vlist = BufReader::new(&f)
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    let ary = vlist[0]
        .split(",")
        .map(|num| num.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let turns = vec![vec![3, 1], vec![0, 2], vec![1, 3], vec![2, 0]];
    let delta = vec![vec![0, -1], vec![1, 0], vec![0, 1], vec![-1, 0]];

    {
        let mut pos_x = 0i64;
        let mut pos_y = 0i64;
        let mut facing = 0; // 0=up, 1=right, 2=down, 3=left
        let mut panels = HashMap::new();

        let mut c = Computer::new(&ary);
        c.run();

        while !c.halted() {
            assert!(c.waiting_for_input());

            let tuple = (pos_x, pos_y);
            if panels.contains_key(&tuple) {
                let panel_color = panels.get(&tuple).unwrap();
                c.run_with_input(*panel_color);
            } else {
                c.run_with_input(0); // panels start out as black
            }
            let new_color = c.next_output();
            let new_direction = c.next_output() as usize;

            panels.insert(tuple, new_color);

            facing = turns[facing][new_direction];
            pos_x += delta[facing][0];
            pos_y += delta[facing][1];
        }
        println!("part 1 = {}", panels.len());
    }

    {
        let mut pos_x = 0i64;
        let mut pos_y = 0i64;

        let mut smallest_x = 0i64;
        let mut smallest_y = 0i64;
        let mut largest_x = 0i64;
        let mut largest_y = 0i64;

        let mut facing = 0; // 0=up, 1=right, 2=down, 3=left
        let mut panels = HashMap::new();

        panels.insert((0, 0), 1);
        let mut c = Computer::new(&ary);
        c.run();

        while !c.halted() {
            assert!(c.waiting_for_input());

            let tuple = (pos_x, pos_y);
            if panels.contains_key(&tuple) {
                let panel_color = panels.get(&tuple).unwrap();
                c.run_with_input(*panel_color);
            } else {
                c.run_with_input(0); // panels start out as black
            }
            let new_color = c.next_output();
            let new_direction = c.next_output() as usize;

            panels.insert(tuple, new_color);

            facing = turns[facing][new_direction];
            pos_x += delta[facing][0];
            pos_y += delta[facing][1];
            if pos_x < smallest_x {
                smallest_x = pos_x;
            }
            if pos_x > largest_x {
                largest_x = pos_x;
            }
            if pos_y < smallest_y {
                smallest_y = pos_y;
            }
            if pos_y > largest_y {
                largest_y = pos_y;
            }
        }
        println!(
            "part 2 = ({},{})-({},{})",
            smallest_x, smallest_y, largest_x, largest_y
        );
        for y in smallest_y..=largest_y {
            for x in smallest_x..=largest_x {
                if panels.contains_key(&(x, y)) {
                    if *panels.get(&(x, y)).unwrap() == 0 {
                        print!(".");
                    } else {
                        print!("#");
                    }
                } else {
                    print!(".");
                }
            }
            println!("");
        }
    }
}
