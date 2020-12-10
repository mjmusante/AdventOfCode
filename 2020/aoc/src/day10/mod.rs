use aoc::utils::nums;

pub fn run() {
    let nums = nums("data/10.txt");
    println!("Part 1 = {}", ones_and_threes(&nums));
    println!("Part 2 = {}", count_arrangements(&nums));
}

fn ones_and_threes(v: &Vec<i64>) -> i64 {
    let mut local = v.clone();
    let mut count1 = 0;
    let mut count3 = 1;

    local.sort();
    let mut num = 0;
    for i in local {
        match i - num {
            1 => count1 += 1,
            3 => count3 += 1,
            2 => (),
            x => {
                panic!(format!("invalid delta {} between {} and {}", x, num, i));
            }
        };
        num = i;
    }

    count1 * count3
}

fn count_arrangements(v: &Vec<i64>) -> i64 {
    let mut local = v.clone();
    let mut result = 1;

    local.sort();
    let plug = local.last().expect("error - empty array").clone();
    local.push(plug + 3);

    let mut last = 0;
    let mut streak = 0;
    for i in local {
        if i - last == 1 {
            streak += 1;
        } else if i - last == 2 {
            panic!("noo!");
        } else {
            match streak {
                0 | 1 => (),
                2 => result *= 2,
                3 => result *= 4,
                4 => result *= 7,
                i => {
                    panic!(format!("Streak of {} is too big", i));
                }
            }
            streak = 0;
        }
        last = i;
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_data1() -> Vec<i64> {
        vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4]
    }

    fn test_data2() -> Vec<i64> {
        vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ]
    }

    #[test]
    fn test1() {
        assert_eq!(ones_and_threes(&test_data1()), 35);
        assert_eq!(ones_and_threes(&test_data2()), 220);
    }

    #[test]
    fn test2() {
        assert_eq!(count_arrangements(&test_data1()), 8);
        assert_eq!(count_arrangements(&test_data2()), 19208);
    }
}
