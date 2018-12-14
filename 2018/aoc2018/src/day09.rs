use lines;
use std::collections::VecDeque;

pub fn run() -> (String, String) {
    let data = lines::lineread(String::from("puzzle_data/day09.txt"))[0]
        .split(" ")
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    let (players, marbles) = (
        data[0].parse::<u64>().unwrap(),
        data[6].parse::<u64>().unwrap(),
    );

    (
        part1(players, marbles).to_string(),
        part1(players, 100 * marbles).to_string(),
    )
}

fn part1(players: u64, marbles: u64) -> u64 {
    let mut cur_player = 0;
    let mut v = vec![0; players as usize];
    let mut circle: VecDeque<u64> = VecDeque::new();

    circle.push_back(0);

    for i in 1..(marbles + 1) {
        if i % 23 == 0 {
            for _ in 0..7 {
                if let Some(popped) = circle.pop_back() {
                    circle.push_front(popped);
                }
            }
            // circle.rotate_right(7);
            v[cur_player] += i + circle.pop_back().unwrap();
            let popped = circle.pop_front().unwrap();
            circle.push_back(popped);
        } else {
            let popped = circle.pop_front().unwrap();
            circle.push_back(popped);
            circle.push_back(i);
        }
        // println!("[{}] {:?}", cur_player + 1, circle);
        cur_player = (cur_player + 1) % (players as usize);
    }

    *v.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day09_test1() {
        assert_eq!(part1(9, 25), 32);
    }

    #[test]
    fn day09_test2() {
        assert_eq!(part1(10, 1618), 8317);
    }

    #[test]
    fn day09_test3() {
        assert_eq!(part1(17, 1104), 2764);
    }

    #[test]
    fn day09_test4() {
        assert_eq!(part1(13, 7999), 146373);
    }

    #[test]
    fn day09_test5() {
        assert_eq!(part1(21, 6111), 54718);
    }

    #[test]
    fn day09_test6() {
        assert_eq!(part1(30, 5807), 37305);
    }
}
