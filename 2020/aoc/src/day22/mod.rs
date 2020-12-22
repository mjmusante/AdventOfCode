use std::collections::VecDeque;

use aoc::utils::lines;

pub fn run() {
    let lines = lines("data/22.txt");
    let (h1, h2) = parse(&lines);

    println!("Part 1 = {}", score(&h1, &h2));
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
    }
}
