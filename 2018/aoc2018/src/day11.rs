pub fn run() -> (String, String) {
    let grid = make_grid(7400);
    (
        format!("{:?}", highest_power(&grid, 3)),
        format!("{:?}", find_best_for(&grid)),
    )
}

const SIZE: usize = 300;

fn highest_power(grid: &Vec<Vec<i64>>, gridsize: usize) -> ((i64, i64), i64) {
    let mut best: i64 = -5 * (gridsize * gridsize) as i64;
    let mut loc = (0i64, 0i64);

    for row in 0..(SIZE - gridsize + 1) {
        for col in 0..(SIZE - gridsize + 1) {
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

    (loc, best)
}

fn find_best_for(grid: &Vec<Vec<i64>>) -> (i64, i64, usize) {
    let mut bestloc = (0, 0);
    let mut bestpower: i64 = 300 * 300 * -5;
    let mut bestsize = 0;

    for s in 1..(SIZE - 1) {
        let msize = SIZE - s;
        if bestpower > (4 * msize * msize) as i64 {
            break;
        }
        let (loc, power) = highest_power(grid, msize);
        if power > bestpower {
            bestsize = msize;
            bestloc = loc;
            bestpower = power;
        }
    }
    (bestloc.0, bestloc.1, bestsize)
}

fn get_pl(ser: i64, x: i64, y: i64) -> i64 {
    let rack_id = x + 10;
    ((rack_id * y + ser) * rack_id) / 100 % 10 - 5
}

fn make_grid(ser: i64) -> Vec<Vec<i64>> {
    let mut grid = vec![vec![0; SIZE]; SIZE];
    for x in 0..SIZE {
        for y in 0..SIZE {
            grid[y][x] = get_pl(ser, x as i64, y as i64);
        }
    }

    grid
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
        assert_eq!(highest_power(&make_grid(18), 3).0, (33, 45));
        assert_eq!(highest_power(&make_grid(42), 3).0, (21, 61));
    }

    #[test]
    fn day11_test3() {
        assert_eq!(find_best_for(&make_grid(18)), (90, 269, 16));
    }
}
