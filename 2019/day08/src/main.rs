use std::process::exit;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

const LAYER_SIZE: usize = 150;
const WIDTH: usize = 25;
const HEIGHT: usize = 6;

const PIXEL_BLACK: char = '.';
const PIXEL_WHITE: char = '#';

fn main() {
    let f = File::open("inputs/day08.txt").unwrap();
    let vlist = BufReader::new(&f)
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    let ary = vlist[0]
        .chars()
        .map(|c| match c {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            _ => {
                println!("invalid char in input {}", c);
                exit(1);
            }
        })
        .collect::<Vec<i64>>();

    let max = ary.len() / LAYER_SIZE;

    let mut counts = vec![0; max];
    for row in 0..max {
        for col in 0..LAYER_SIZE {
            if ary[LAYER_SIZE * row + col] == 0 {
                counts[row] += 1;
            }
        }
    }

    let mut smallest = max;
    let mut smindex = max;
    for i in 0..counts.len() {
        if counts[i] < smallest {
            smallest = counts[i];
            smindex = i;
        }
    }

    if smindex == max {
        println!("no smallest entry found?!");
        exit(1);
    }

    let (mut count1, mut count2) = (0, 0);
    for i in 0..LAYER_SIZE {
        match ary[LAYER_SIZE * smindex + i] {
            1 => count1 += 1,
            2 => count2 += 1,
            0 => { /* ignore */ }
            _ => {
                println!("invalid match");
                exit(1);
            }
        }
    }

    println!("part 1 = {}", count1 * count2);

    let mut image = vec![vec!['_'; WIDTH]; HEIGHT];

    for pixel in 0..LAYER_SIZE {
        for depth in 0..max {
            let val = ary[LAYER_SIZE * depth + pixel];
            let (row, col) = (pixel / WIDTH, pixel % WIDTH);
            match val {
                0 => {
                    image[row][col] = PIXEL_BLACK;
                    break;
                }
                1 => {
                    image[row][col] = PIXEL_WHITE;
                    break;
                }
                _ => { /* continue */ }
            }
        }
    }

    println!("part 2:");
    for row in image {
        for col in row {
            print!("{}", col);
        }
        println!("");
    }
}
