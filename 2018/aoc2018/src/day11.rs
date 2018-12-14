pub fn run() -> (String, String) {
    (format!("{:?}", highest_power(7400, 3)), "world".to_string())
}

const SIZE : usize = 300;

fn highest_power(ser: i64, gridsize: usize) -> (i64, i64) {
    let mut grid = vec![vec![0; SIZE]; SIZE];
    for x in 0..SIZE {
        for y in 0..SIZE {
            grid[y][x] = get_pl(ser, x as i64, y as i64);
        }
    }

    let mut best : i64 = -5 * 9;    // each cell ranges from -5 to 4
    let mut loc = (0i64, 0i64);

    for row in 0..(SIZE - gridsize) {
        for col in 0..(SIZE - gridsize) {
            let mut cur_power = 0;
            for r in row..(row + gridsize) {
                for c in col..(col + gridsize) {
                    cur_power += grid[c][r];
                }
            }

            if best < cur_power {
                best = cur_power;
                loc = (row as i64, col as i64);
            }
        }
    }

    loc
}

fn get_pl(ser: i64, x: i64, y: i64) -> i64 {
    let rack_id = x + 10;
    ((rack_id * y + ser) * rack_id) / 100 % 10 - 5
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day11_test1() {
        assert_eq!(get_pl(8, 3, 5), 4);
        assert_eq!(get_pl(57, 122, 79), -5);
        assert_eq!(get_pl(39, 217, 196), 0);
        assert_eq!(get_pl(71, 101, 153), 4);
    }

    #[test]
    fn day11_test2() {
        assert_eq!(highest_power(18, 3), (33, 45));
        assert_eq!(highest_power(42, 3), (21, 61));
    }
}
