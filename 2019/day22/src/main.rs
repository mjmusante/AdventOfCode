extern crate num;
extern crate num_traits;

use num::bigint::BigInt;
use num::ToPrimitive;

use num_traits::{One, Zero};

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use regex::Regex;

const SIZE: usize = 10007;

fn shuffle(start: &Vec<usize>, rules: &Vec<String>) -> Vec<usize> {
    let deal = Regex::new(r"deal with increment (\d+)").unwrap();
    let rev = Regex::new(r"deal into new stack").unwrap();
    let cut = Regex::new(r"cut (-?\d+)").unwrap();

    let mut deck = start.clone();

    for v in rules {
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

    deck
}

/* stolen from https://stackoverflow.com/questions/8496182/calculating-powa-b-mod-n */
fn modpow(b: u64, e: u64, modulus: u64) -> BigInt {
    let mut exp = e;
    let mut base: BigInt = (b % modulus).into();
    let mut result: BigInt = One::one();
    while exp > 0 {
        if exp & 1 == 1 {
            result = (result * base) % modulus as u128;
        }
        base = (base * base) % modulus as u128;
        exp >>= 1;
    }

    result
}

fn reverse_compute(cards: u64, shuffles: u64, index: u64, rules: &Vec<String>) -> u64 {
    let one: BigInt = One::one();
    let mut val: BigInt = One::one();
    let mut o: BigInt = Zero::zero();

    let deal = Regex::new(r"deal with increment (\d+)").unwrap();
    let rev = Regex::new(r"deal into new stack").unwrap();
    let cut = Regex::new(r"cut (-?\d+)").unwrap();

    for r in rules {
        if deal.is_match(&r) {
            let cap = deal.captures_iter(&r).next().unwrap();
            let incr = cap[1].parse::<u64>().unwrap();
            let newval = val * modpow(incr, cards - 2, cards);
            val = newval;
        } else if rev.is_match(&r) {
            o -= val;
            val *= -1;
        } else if cut.is_match(&r) {
            let cap = cut.captures_iter(&r).next().unwrap();
            let dist = cap[1].parse::<i128>().unwrap();
            o += val * dist;
        }
    }
    o *= modpow((one - val).to_u64().unwrap(), cards - 2, cards);
    val = modpow(val.to_u64().unwrap(), shuffles, cards);

    let mul: BigInt = (one - val) * o;

    (index * val).to_u64().unwrap() + mul.to_u64().unwrap() % cards
}

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

    let newdeck = shuffle(&deck, &vlist);

    for i in 0..10007 {
        if newdeck[i] == 2019 {
            println!("part 1 = {}", i);
            break;
        }
    }

    println!(
        "part 2 = {}",
        reverse_compute(119315717514047, 101741582076661, 2020, &vlist)
    );
}
