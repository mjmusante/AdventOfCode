use std::cmp::{max, min};

use std::collections::HashMap;

use intcode::Computer;

struct Robot {
    program: Computer,
    facing: usize,
    panels: HashMap<(i64, i64), i64>,
    pos_x: i64,
    pos_y: i64,
    upper_left: (i64, i64),
    lower_right: (i64, i64),
}

impl Robot {
    const TURNS: [[i64; 2]; 4] = [[3, 1], [0, 2], [1, 3], [2, 0]];
    const MOVES: [[i64; 2]; 4] = [[0, -1], [1, 0], [0, 1], [-1, 0]];

    pub fn new(c: Computer) -> Robot {
        Robot {
            program: c,
            facing: 0,
            panels: HashMap::new(),
            pos_x: 0,
            pos_y: 0,
            upper_left: (0, 0),
            lower_right: (0, 0),
        }
    }

    pub fn run(&mut self) {
        self.program.run();

        while !self.program.halted() {
            assert!(self.program.waiting_for_input());

            let tuple = (self.pos_x, self.pos_y);
            if self.panels.contains_key(&tuple) {
                let panel_color = self.panels.get(&tuple).unwrap();
                self.program.run_with_input(*panel_color);
            } else {
                self.program.run_with_input(0); // panels start out as black
            }
            let new_color = self.program.next_output();
            let new_direction = self.program.next_output() as usize;

            self.panels.insert(tuple, new_color);

            self.facing = Robot::TURNS[self.facing][new_direction] as usize;
            self.pos_x += Robot::MOVES[self.facing][0];
            self.pos_y += Robot::MOVES[self.facing][1];

            self.upper_left = (
                min(self.upper_left.0, self.pos_x),
                min(self.upper_left.1, self.pos_y),
            );
            self.lower_right = (
                max(self.lower_right.0, self.pos_x),
                max(self.lower_right.1, self.pos_y),
            );
        }
    }

    pub fn visited(&self) -> usize {
        self.panels.len()
    }

    pub fn paint_one(&mut self, loc: (i64, i64), color: i64) {
        self.panels.insert(loc, color);
    }

    pub fn print_panels(&self) {
        for y in self.upper_left.1..=self.lower_right.1 {
            for x in self.upper_left.0..=self.lower_right.0 {
                if !self.panels.contains_key(&(x, y)) || *self.panels.get(&(x, y)).unwrap() == 0 {
                    print!(".");
                } else {
                    print!("#");
                }
            }
            println!("");
        }
    }
}

fn main() {
    let c1 = Computer::new().from_file("inputs/day11.txt");
    let mut r1 = Robot::new(c1);
    r1.run();
    println!("part 1 = {}", r1.visited());

    let c2 = Computer::new().from_file("inputs/day11.txt");
    let mut r2 = Robot::new(c2);
    r2.paint_one((0, 0), 1);
    r2.run();
    println!("part 2:");
    r2.print_panels();
}
