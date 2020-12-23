use std::collections::{HashSet, VecDeque};

pub fn run() {
    let puzzle_input = 219347865;

    println!("Part 1 = {}", play_cups(puzzle_input, 100));
}

fn play_cups(order: i64, rounds: i64) -> i64 {
    let mut v = VecDeque::new();
    let mut stack = Vec::new();
    let mut remainder = order;

    while remainder > 0 {
        stack.push(remainder % 10);
        remainder /= 10;
    }

    while !stack.is_empty() {
        v.push_back(stack.pop().unwrap());
    }

    // println!(" i: {:?}", v);

    for _ in 0..rounds {
        let cur_cup = v.pop_front().unwrap();
        v.push_back(cur_cup);

        let (a, b, c) = (
            v.pop_front().unwrap(),
            v.pop_front().unwrap(),
            v.pop_front().unwrap(),
        );
        let next_cup = *v.get(0).unwrap();

        let mut find = cur_cup;
        while find == a || find == b || find == c || find == cur_cup {
            find = if find == 1 { 9 } else { find - 1 };
        }
        let mut next = v.pop_front().unwrap();
        while next != find {
            v.push_back(next);
            next = v.pop_front().unwrap();
        }
        v.push_back(next);
        v.push_back(a);
        v.push_back(b);
        v.push_back(c);
        while *v.get(0).unwrap() != next_cup {
            let x = v.pop_front().unwrap();
            v.push_back(x);
        }
        // println!("{:2}: {:?}", i + 2, v);
    }

    loop {
        let tip = v.pop_front().unwrap();
        if tip == 1 {
            break;
        }
        v.push_back(tip);
    }

    let mut result = 0;
    while !v.is_empty() {
        result = 10 * result + v.pop_front().unwrap();
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(play_cups(389125467, 10), 92658374);
        assert_eq!(play_cups(389125467, 100), 67384529);
    }
}
