use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use regex::Regex;

#[derive(Clone, Debug)]
struct Moon {
    xpos: i64,
    ypos: i64,
    zpos: i64,
    xvel: i64,
    yvel: i64,
    zvel: i64,
}

impl Moon {
    pub fn new(x: i64, y: i64, z: i64) -> Moon {
        Moon {
            xpos: x,
            ypos: y,
            zpos: z,
            xvel: 0,
            yvel: 0,
            zvel: 0,
        }
    }

    pub fn gravity(&mut self, other: &Moon) {
        if self.xpos < other.xpos {
            self.xvel += 1;
        } else if self.xpos > other.xpos {
            self.xvel -= 1;
        }
        if self.ypos < other.ypos {
            self.yvel += 1;
        } else if self.ypos > other.ypos {
            self.yvel -= 1;
        }
        if self.zpos < other.zpos {
            self.zvel += 1;
        } else if self.zpos > other.zpos {
            self.zvel -= 1;
        }
    }

    pub fn apply(&mut self) {
        self.xpos += self.xvel;
        self.ypos += self.yvel;
        self.zpos += self.zvel;
    }

    pub fn energy(&self) -> i64 {
        (self.xpos.abs() + self.ypos.abs() + self.zpos.abs())
            * (self.xvel.abs() + self.yvel.abs() + self.zvel.abs())
    }
}

fn main() {
    let f = File::open("inputs/day12.txt").unwrap();
    let vlist = BufReader::new(&f)
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    let reg = Regex::new(r"<x=(-*\d+), y=(-*\d+), z=(-*\d+)>").unwrap();
    let mut moons = vec![];

    for v in &vlist {
        let cap = reg.captures_iter(v).next().unwrap();
        let xpos = cap[1].parse::<i64>().unwrap();
        let ypos = cap[2].parse::<i64>().unwrap();
        let zpos = cap[3].parse::<i64>().unwrap();
        moons.push(Moon::new(xpos, ypos, zpos));
    }

    for _ in 0..1000 {
        let mut newmoons = vec![];
        for i in 0..moons.len() {
            let mut m = moons[i].clone();
            for j in 0..moons.len() {
                if i != j {
                    m.gravity(&moons[j]);
                }
            }
            m.apply();
            newmoons.push(m);
        }
        moons = newmoons;
    }

    let total_energy: i64 = moons.iter().map(|m| m.energy()).sum();
    println!("part 1 = {}", total_energy);
}
