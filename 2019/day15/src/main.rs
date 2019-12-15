use std::collections::HashMap;

use std::cmp::{max, min};

use intcode::Computer;

fn _show_maze(
    maze: &HashMap<(i64, i64), char>,
    cur: &(i64, i64),
    upper_left: &(i64, i64),
    lower_right: &(i64, i64),
) {
    for y in upper_left.1..=lower_right.1 {
        for x in upper_left.0..=lower_right.0 {
            if x == 0 && y == 0 {
                print!("X");
            } else if x == cur.0 && y == cur.1 {
                print!("@");
            } else if maze.contains_key(&(x, y)) {
                let ch = *maze.get(&(x, y)).unwrap();
                print!("{}", ch);
            } else {
                print!(" ");
            }
        }
        println!("");
    }
}

fn main() {
    let dir_to_pos = vec![(0, -1), (0, 1), (-1, 0), (1, 0)];
    let reverse = vec![1, 0, 3, 2];

    let mut c = Computer::new().from_file("inputs/day15.txt");

    let mut maze = HashMap::new();
    let mut dir: usize = 0;
    let mut pos = (0, 0);
    let mut stack: Vec<((i64, i64), usize)> = vec![];
    let mut upper_left = (0, 0);
    let mut lower_right = (0, 0);

    maze.insert((0, 0), '.');

    while !c.halted() {
        let result;

        let nextpos = (pos.0 + dir_to_pos[dir].0, pos.1 + dir_to_pos[dir].1);
        if maze.contains_key(&nextpos) {
            result = -1;
        } else {
            c.run_with_input(dir as i64 + 1);
            result = c.next_output();
        }
        if result < 1 {
            // hit a wall
            if result == 0 {
                maze.insert(nextpos, '#');
                upper_left = (min(upper_left.0, nextpos.0), min(upper_left.1, nextpos.1));
                lower_right = (max(lower_right.0, nextpos.0), max(lower_right.1, nextpos.1));
            }
            dir = (dir + 1) % 4;
            while dir == 0 && stack.len() > 0 {
                // tried every direction; back up one and try again
                let m = stack.pop().unwrap();
                c.run_with_input(m.1 as i64 + 1);
                let r = c.next_output();
                assert_eq!(r, 1);
                pos = m.0;
                dir = (reverse[m.1] + 1) % 4;
            }
            if dir == 0 && result == -1 {
                // we're back home
                break;
            }
        } else if result == 1 {
            // successful move
            stack.push((pos, reverse[dir]));
            maze.insert(nextpos, '.');
            pos = nextpos;
            dir = 0;
            upper_left = (min(upper_left.0, pos.0), min(upper_left.1, pos.1));
            lower_right = (max(lower_right.0, pos.0), max(lower_right.1, pos.1));
        } else {
            // found oxygen system
            assert_eq!(result, 2);
            stack.push((pos, reverse[dir]));
            maze.insert(pos, 'O');
            println!("part 1 = {}", stack.len());
        }
    }
}
