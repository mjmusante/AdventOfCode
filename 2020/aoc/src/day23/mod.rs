use std::collections::VecDeque;

pub fn run() {
    let puzzle_input = 219347865;

    println!("Part 1 = {}", play_cups(puzzle_input, false, 100));
    println!("Part 2 = {}", play_cups(puzzle_input, true, 10_000_000));
}

fn convert(num: i64) -> VecDeque<i64> {
    let mut v = VecDeque::new();
    let mut stack = Vec::new();
    let mut remainder = num;

    while remainder > 0 {
        stack.push(remainder % 10);
        remainder /= 10;
    }

    while !stack.is_empty() {
        v.push_back(stack.pop().unwrap());
    }

    v
}

fn print_cups(list: &[i64], len: usize, start: usize) {
    let mut ptr = start;
    print!("cups:");
    for i in 0..len {
        print!(" {}", ptr + 1);
        ptr = list[ptr] as usize;
    }
    println!("");
}

fn play_cups(order: i64, extend: bool, rounds: i64) -> i64 {
    let mut v = convert(order);

    if extend {
        let mut cupvec = Vec::<i64>::new();
        let total_cups = 1_000_000;
        for i in 0..total_cups {
            // for i in 0..9 {
            cupvec.push(i as i64 + 1);
        }

        let mut cups = cupvec.into_boxed_slice();
        let count = v.len();
        for i in 0..count - 1 {
            // println!("cup {} points to cup {}", v[i] - 1, v[i + 1] - 1);
            cups[v[i] as usize - 1] = v[i + 1] - 1;
        }
        cups[v[count - 1] as usize - 1] = count as i64;
        cups[total_cups - 1] = v[0] - 1;

        // println!("cups ==> {:?}", cups);

        let mut cur_cup = v[0] - 1;
        for i in 0..rounds {
            // print!("{:2}: ", i + 1);
            // print_cups(&cups, count, cur_cup as usize);

            let a = cups[cur_cup as usize];
            let b = cups[a as usize];
            let c = cups[b as usize];
            let next_cup = cups[c as usize];

            let mut find = cur_cup;
            while find == a || find == b || find == c || find == cur_cup {
                find = if find == 0 {
                    total_cups as i64 - 1
                } else {
                    find - 1
                };
            }

            let tmp = cups[find as usize];
            cups[find as usize] = a;
            cups[cur_cup as usize] = cups[c as usize];
            cups[c as usize] = tmp;
            cur_cup = cups[cur_cup as usize];

            // let tmp = cups[find as usize];
            // cups[cur_cup as usize] = next_cup;
            // cups[find as usize] = a;
            // cups[c as usize] = tmp;
            // cur_cup = next_cup;
        }

        return (cups[0] + 1) * (cups[cups[0] as usize] + 1);
    }

    solve(&mut v, false, rounds)
}

fn solve(v: &mut VecDeque<i64>, extend: bool, rounds: i64) -> i64 {
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

    if extend {
        return 0;
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
        assert_eq!(play_cups(389125467, false, 10), 92658374);
        assert_eq!(play_cups(389125467, false, 100), 67384529);
    }

    #[test]
    fn test2() {
        assert_eq!(play_cups(389125467, true, 10_000_000), 149245887792);
        // assert_eq!(play_cups(389125467, true, 10), 92658374);
        // assert_eq!(play_cups(389125467, false, 100), 67384529);
    }
}
