use std::collections::hash_map::DefaultHasher;
use std::collections::{HashSet, VecDeque};
use std::hash::{Hash, Hasher};

use aoc::utils::lines;

pub fn run() {
    let lines = lines("data/22.txt");
    let (h1, h2) = parse(&lines);

    println!("Part 1 = {}", score(&h1, &h2));
    println!("Part 2 = {}", recursive(&h1, &h2, true).1)
}

fn parse(lines: &Vec<String>) -> (Vec<i64>, Vec<i64>) {
    let mut hand1 = Vec::<i64>::new();
    let mut hand2 = Vec::<i64>::new();
    let mut player1 = true;

    for l in lines {
        if l == "" {
            player1 = false;
            continue;
        }
        if l.starts_with("Player") {
            continue;
        }
        if player1 {
            hand1.push(l.parse().unwrap());
        } else {
            hand2.push(l.parse().unwrap());
        }
    }

    (hand1, hand2)
}

fn score(h1: &Vec<i64>, h2: &Vec<i64>) -> i64 {
    let mut player1 = VecDeque::from(h1.clone());
    let mut player2 = VecDeque::from(h2.clone());

    while player1.len() > 0 && player2.len() > 0 {
        let c1 = player1.pop_front().unwrap();
        let c2 = player2.pop_front().unwrap();

        if c1 > c2 {
            player1.push_back(c1);
            player1.push_back(c2);
        } else {
            player2.push_back(c2);
            player2.push_back(c1);
        }
    }

    let winner = if player1.len() > 0 { player1 } else { player2 };
    let mut mul = winner.len() as i64;
    let mut result = 0;
    for c in winner {
        result += mul * c;
        mul -= 1;
    }

    result
}

fn make_hash(first: &VecDeque<i64>, second: &VecDeque<i64>) -> u64 {
    let mut compute = DefaultHasher::new();
    first.hash(&mut compute);
    second.hash(&mut compute);

    compute.finish()
}

fn recursive(h1: &Vec<i64>, h2: &Vec<i64>, top: bool) -> (bool, i64) {
    let mut player1 = VecDeque::from(h1.clone());
    let mut player2 = VecDeque::from(h2.clone());
    let mut previous = HashSet::new();

    while player1.len() > 0 && player2.len() > 0 {
        // println!("Playing: {:?} {:?}", player1, player2);
        let hval = make_hash(&player1, &player2);
        if previous.contains(&hval) {
            // println!("\t(seen before)");
            return (true, 0);
        }

        previous.insert(hval);

        // println!("Player 1's Deck: {:?}", player1);
        // println!("Player 2's Deck: {:?}", player2);
        let c1 = player1.pop_front().unwrap();
        let c2 = player2.pop_front().unwrap();
        // println!("Player 1 plays: {}", c1);
        // println!("Player 2 plays: {}", c2);
        let mut p1win = c1 > c2;

        if player1.len() as i64 >= c1 && player2.len() as i64 >= c2 {
            // player1.make_contiguous();
            // player2.make_contiguous();
            // let (slice_h1, _) = player1.as_slices();
            // let (slice_h2, _) = player2.as_slices();
            // let new_h1 = Vec::from(&slice_h1[0..c1 as usize]);
            // let new_h2 = Vec::from(&slice_h2[0..c2 as usize]);
            let new_h1: Vec<i64> = player1.iter().take(c1 as usize).copied().collect();
            let new_h2: Vec<i64> = player2.iter().take(c2 as usize).copied().collect();

            let max_h1 = new_h1.iter().max();
            let max_h2 = new_h2.iter().max();
            // println!("== checking subgame ===");
            p1win = max_h1 > max_h2;
            if !p1win {
                let (real_win, _) = recursive(&new_h1, &new_h2, false);
                p1win = real_win;
            }
        }

        if p1win {
            // println!("Player 1 wins!");
            player1.push_back(c1);
            player1.push_back(c2);
        } else {
            // println!("Player 2 wins!");
            player2.push_back(c2);
            player2.push_back(c1);
        }
    }

    if top {
        let winner = if player1.len() > 0 {
            &player1
        } else {
            &player2
        };
        let mut mul = winner.len() as i64;
        let mut result = 0;
        for c in winner {
            result += mul * c;
            mul -= 1;
        }

        return (player1.len() > 0, result);
    }

    (player1.len() > 0, 0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let v = vec![
            "Player 1:".to_string(),
            "9".to_string(),
            "2".to_string(),
            "6".to_string(),
            "3".to_string(),
            "1".to_string(),
            "".to_string(),
            "Player 2:".to_string(),
            "5".to_string(),
            "8".to_string(),
            "4".to_string(),
            "7".to_string(),
            "10".to_string(),
        ];
        let (p1, p2) = parse(&v);
        assert_eq!(score(&p1, &p2), 306);
        assert_eq!(recursive(&p1, &p2, true), (false, 291));
    }
}
