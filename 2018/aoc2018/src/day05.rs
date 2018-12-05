use lines;

fn part1(line: &String) -> usize {
    let mut result = String::new();

    result.push_str(&line);
    loop {
        let mut prev_c = '0';
        let mut prev_lower = false;
        let mut mutated = false;
        let mut newstr = String::new();

        for c in result.chars() {
            let c_lower = c.is_ascii_lowercase();
            if c.to_ascii_uppercase() == prev_c.to_ascii_uppercase() {
                if prev_lower == c_lower {
                    newstr.push(prev_c);
                    prev_c = c;
                    prev_lower = c_lower;
                } else {
                    prev_c = '0';
                    mutated = true;
                }
            } else {
                if prev_c != '0' {
                    newstr.push(prev_c);
                }
                prev_c = c;
                prev_lower = c_lower;
            }
        }
        if prev_c != '0' {
            newstr.push(prev_c);
        }
        result.clear();
        result.push_str(&newstr);

        if !mutated {
            break;
        }
    }

    result.len()
}

fn part2(line: &String) -> usize {
    let mut best = line.len();
    for c in ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
        'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W',
        'X', 'Y', 'Z'].iter() {
        let mut newstr = String::new();
        newstr.push_str(&line.chars()
            .filter(|x| x.to_ascii_uppercase() != *c)
            .collect::<String>());
        let next = part1(&newstr);
        if next < best {
            best = next;
        }
    }
    best
}


pub fn run() {
    let lines = lines::lineread(String::from("puzzle_data/day05.txt"));
    let foo = lines[0].trim().to_string();
    println!("Part 1: {}", part1(&foo));
    println!("Part 2: {}", part2(&foo));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day05_test1() {
        assert_eq!(part1(&"aA".to_string()), 0);
    }

    #[test]
    fn day05_test2() {
        assert_eq!(part1(&"abBA".to_string()), 0);
    }

    #[test]
    fn day05_test3() {
        assert_eq!(part1(&"abAB".to_string()), 4);
    }

    #[test]
    fn day05_test4() {
        assert_eq!(part1(&"aabAAB".to_string()), 6);
    }

    #[test]
    fn day05_test5() {
        assert_eq!(part1(&"dabAcCaCBAcCcaDA".to_string()), 10);
    }

    #[test]
    fn day05_test6() {
        assert_eq!(part2(&"dabAcCaCBAcCcaDA".to_string()), 4);
    }
}
