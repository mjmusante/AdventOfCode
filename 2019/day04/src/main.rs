fn valid(num: i64, exact: bool) -> bool {
    let mut prev_dig = 10;
    let mut left = num;
    let mut doubles = false;
    let mut dubcount = 0;

    while left > 0 {
        let dig = left % 10;
        left /= 10;
        if exact {
            if dig == prev_dig {
                dubcount += 1;
            } else if dubcount == 1 {
                doubles = true;
            } else {
                dubcount = 0;
            }
        } else {
            if dig == prev_dig {
                doubles = true;
            }
        }
        if dig > prev_dig {
            return false;
        }
        prev_dig = dig;
    }

    doubles || dubcount == 1
}

fn main() {
    assert!(valid(122345, false));
    assert!(valid(111123, false));
    assert!(valid(111111, false));
    assert!(!valid(135679, false));
    assert!(!valid(223450, false));
    assert!(!valid(123789, false));

    assert!(valid(112233, true));
    assert!(!valid(123444, true));
    assert!(valid(111122, true));

    assert!(!valid(444555, true));
    assert!(!valid(144445, true));
    assert!(valid(112345, true));

    let mut part1 = 0;
    let mut part2 = 0;
    for i in 130254..=678275 {
        if valid(i, false) {
            part1 += 1;
        }
        if valid(i, true) {
            part2 += 1;
        }
    }
    println!("part 1: {}", part1);
    println!("part 2: {}", part2);
}
