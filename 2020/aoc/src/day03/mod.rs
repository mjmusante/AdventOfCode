use aoc::utils::lines;

use aoc::grid::Grid2D;

pub fn run() {
    let grid = Grid2D::new_from_file("data/03.txt");

    let part1 = count_2d(&grid, 3, 1);
    println!("Part 1 = {}", part1);

    let slopes = vec![(1, 1), (5, 1), (7, 1), (1, 2)];
    let part2 = slopes
        .iter()
        .map(|(right, down)| count_2d(&grid, *right, *down))
        .fold(part1, |prod, x| prod * x);
    println!("Part 2 = {}", part2);
}

fn count_2d(grid: &Grid2D, right: i64, down: usize) -> usize {
    let mut count = 0;
    let mut col = 0;

    for row in grid.rows().step_by(down) {
        if grid.contains(row, col) {
            count += 1;
        }
        col = (col + right) % grid.col_count();
    }

    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let v = vec![
            "..##.......".to_string(),
            "#...#...#..".to_string(),
            ".#....#..#.".to_string(),
            "..#.#...#.#".to_string(),
            ".#...##..#.".to_string(),
            "..#.##.....".to_string(),
            ".#.#.#....#".to_string(),
            ".#........#".to_string(),
            "#.##...#...".to_string(),
            "#...##....#".to_string(),
            ".#..#...#.#".to_string(),
        ];
        assert_eq!(count(&v, 3, 1), 7);

        let g2d = Grid2D::new_from_vec(&v);
        assert_eq!(count_2d(&g2d, 3, 1), 7);
    }

    #[test]
    fn test2() {
        let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
        let v = vec![
            "..##.......".to_string(),
            "#...#...#..".to_string(),
            ".#....#..#.".to_string(),
            "..#.#...#.#".to_string(),
            ".#...##..#.".to_string(),
            "..#.##.....".to_string(),
            ".#.#.#....#".to_string(),
            ".#........#".to_string(),
            "#.##...#...".to_string(),
            "#...##....#".to_string(),
            ".#..#...#.#".to_string(),
        ];

        let g2d = Grid2D::new_from_vec(&v);
        assert_eq!(
            slopes
                .iter()
                .map(|(right, down)| count_2d(&g2d, *right, *down))
                .fold(1, |prod, x| prod * x),
            336
        );
    }
}
