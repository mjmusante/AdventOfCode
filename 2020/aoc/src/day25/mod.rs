use aoc::utils::nums;

pub fn run() {
    let vals = nums("data/25.txt");

    println!("Part 1 = {}", solve(vals[0], vals[1]));

}


fn find_loop_of(key: i64) -> i64 {
    let mut ans = 7;
    let mut count = 1;

    while ans != key {
        ans = (ans * 7) % 20201227;
        count += 1;
    }

    count
}

fn solve(door_pubkey: i64, card_pubkey: i64) -> i64 {
    let card_loop = find_loop_of(card_pubkey);
    let mut ans = door_pubkey;

    for _ in 1..card_loop {
        ans = (ans * door_pubkey) % 20201227;
    }

    ans
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(find_loop_of(17807724), 11);
        assert_eq!(find_loop_of(5764801), 8);

    }

    #[test]
    fn test2() {
        assert_eq!(solve(17807724, 5764801), 14897079);
    }
}
