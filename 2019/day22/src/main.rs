use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use regex::Regex;

const SIZE: usize = 10007;

fn main() {
    let f = File::open("inputs/day22.txt").unwrap();
    let vlist = BufReader::new(&f)
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();
    let mut deck = vec![];
    for i in 0..SIZE {
        deck.push(i);
    }

    let deal = Regex::new(r"deal with increment (\d+)").unwrap();
    let rev = Regex::new(r"deal into new stack").unwrap();
    let cut = Regex::new(r"cut (-?\d+)").unwrap();

    for v in vlist {
        if deal.is_match(&v) {
            let cap = deal.captures_iter(&v).next().unwrap();
            let incr = cap[1].parse::<usize>().unwrap();
            let mut newdeck = deck.clone();
            for i in 0..SIZE {
                newdeck[(i * incr) % SIZE] = deck[i];
            }
            deck = newdeck;
        } else if rev.is_match(&v) {
            deck.reverse();
        } else if cut.is_match(&v) {
            let cap = cut.captures_iter(&v).next().unwrap();
            let dist = cap[1].parse::<i64>().unwrap();
            let slice;
            if dist < 0 {
                slice = SIZE - (dist.abs() as usize);
            } else {
                slice = dist as usize;
            }
            let mut newdeck = deck[slice..].to_vec();
            newdeck.extend(deck[0..slice].to_vec());
            deck = newdeck;
        } else {
            println!("NO MATCH: {}", v);
        }
    }

    if deck.len() > 10 {
        for i in 0..10007 {
            if deck[i] == 2019 {
                println!("part 1 = {}", i);
                break;
            }
        }
    } else {
        println!("{:?}", deck);
    }
}
