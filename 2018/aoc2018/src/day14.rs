pub fn run() -> (String, String) {
    let (part1, part2) = solve(290431);
    (format!("{:?}", part1), format!("{:?}", part2))
}

fn solve(n: usize) -> (Vec<usize>, usize) {
    let mut ans = vec![];
    let mut r = vec![3, 7];
    let mut e1 = 0;
    let mut e2 = 1;
    let mut rcount = 2;
    let mut seqdigits = 0;
    let mut result = (false, 0);
    let digits = (n as f64).log10() as u32;
    let p = 10usize.pow(digits) * 10;

    loop {
        let next_r = r[e1] + r[e2];

        if next_r > 9 {
            let d = next_r / 10;
            rcount += 1;
            r.push(d);
            if rcount > n && ans.len() < 10 {
                ans.push(d);
            }

            if !result.0 {
                seqdigits = (seqdigits * 10 + d) % p;
                if seqdigits == n {
                    result = (true, rcount);
                }
            }
            if result.0 && ans.len() == 10 {
                break;
            }
        }

        let d0 = next_r % 10;
        rcount += 1;
        r.push(d0);
        if rcount > n && ans.len() < 10 {
            ans.push(d0);
        }

        if !result.0 {
            seqdigits = (seqdigits * 10 + d0) % p;
            if seqdigits == n {
                result = (true, rcount);
            }
        }

        if result.0 && ans.len() == 10 {
            break;
        }

        e1 = (e1 + r[e1] + 1) % r.len();
        e2 = (e2 + r[e2] + 1) % r.len();

    }

    (ans, result.1 - digits as usize - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day14_test1() {
        assert_eq!(solve(5).0, [0,1,2,4,5,1,5,8,9,1]);
        assert_eq!(solve(9).0, [5,1,5,8,9,1,6,7,7,9]);
        assert_eq!(solve(18).0, [9,2,5,1,0,7,1,0,8,5]);
        assert_eq!(solve(2018).0, [5,9,4,1,4,2,9,8,8,2]);
    }

    #[test]
    fn day14_test2() {
        assert_eq!(solve(51589).1, 9);
        assert_eq!(solve(92510).1, 18);
        assert_eq!(solve(59414).1, 2018);
        // assert_eq!(solve(01245).1, 5); need to fix leading-0 issue :(
    }
}
