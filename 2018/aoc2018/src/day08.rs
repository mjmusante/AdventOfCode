use lines;

fn blergh(thing: &mut Iterator<Item = &u64>) -> u64 {
    let children = thing.next().unwrap();
    let metadata = thing.next().unwrap();
    let mut result = 0;

    for _ in 0..*children {
        result += blergh(thing);
    }

    for _ in 0..*metadata {
        result += *thing.next().unwrap();
    }

    result
}

fn blagh(thing: &mut Iterator<Item = &u64>) -> u64 {
    let children = *thing.next().unwrap() as usize;
    let metadata = *thing.next().unwrap();
    let mut result = 0;

    let mut child = vec![];
    for _ in 0..children {
        child.push(blagh(thing));
    }
    if children > 0 {
        for _ in 0..metadata {
            let i = *thing.next().unwrap() as usize;
            if i > 0 && i <= children {
                result += child[i - 1];
            }
        }    
    } else {
        for _ in 0..metadata {
            result += *thing.next().unwrap();
        }
    }

    result
}

fn part1(data: &Vec<u64>) -> u64 {
    blergh(&mut data.into_iter())
}

fn part2(data: &Vec<u64>) -> u64 {
    blagh(&mut data.into_iter())
}

fn text_to_array(tree: &String) -> Vec<u64> {
    tree.split(" ").map(|x| x.parse::<u64>().unwrap()).collect()
}

pub fn run() -> (String, String) {
    let lines = lines::lineread(String::from("puzzle_data/day08.txt"));
    let ary = text_to_array(&lines[0]);

    (part1(&ary).to_string(), part2(&ary).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day08_test1() {
        let v = text_to_array(&"2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2".to_string());
        assert_eq!(part1(&v), 138);
    }

    #[test]
    fn day08_test2() {
        let v = text_to_array(&"2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2".to_string());
        assert_eq!(part2(&v), 66);
    }
}
