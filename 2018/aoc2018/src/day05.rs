use lines;

fn part1(line: &String) -> usize {
    let mut prev_c = '0';
    let mut prev_lower = false;
    let mut newstr: Vec<char> = vec![];

    for c in line.chars() {
        let c_lower = c.is_ascii_lowercase();
        if c.to_ascii_uppercase() == prev_c.to_ascii_uppercase() {
            if prev_lower == c_lower {
                newstr.push(prev_c);
                prev_c = c;
                prev_lower = c_lower;
            } else {
                if newstr.len() > 0 {
                    prev_c = newstr.last().cloned().unwrap();
                    prev_lower = prev_c.is_ascii_lowercase();
                    let l = newstr.len();
                    newstr.truncate(l - 1);
                } else {
                    prev_c = '0';
                }
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

    newstr.len()
}

fn part2(line: &String) -> usize {
    let mut best = line.len();
    for c in [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ]
        .iter()
    {
        let mut newstr = String::new();
        newstr.push_str(
            &line
                .chars()
                .filter(|x| x.to_ascii_uppercase() != *c)
                .collect::<String>(),
        );
        let next = part1(&newstr);
        if next < best {
            best = next;
        }
    }
    best
}

pub fn run() -> (String, String) {
    let lines = lines::lineread(String::from("puzzle_data/day05.txt"));
    let foo = lines[0].trim().to_string();
    (part1(&foo).to_string(), part2(&foo).to_string())
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
