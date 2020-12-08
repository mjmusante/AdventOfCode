use aoc::utils::lines;

pub fn run() {
    let lines = lines("data/03.txt");

    let part1 = count(&lines, 3, 1);
    println!("Part 1 = {}", part1);

    let slopes = vec![(1, 1), (5, 1), (7, 1), (1, 2)];
    let part2 = slopes
        .iter()
        .map(|(right, down)| count(&lines, *right, *down))
        .fold(part1, |prod, x| prod * x);
    println!("Part 2 = {}", part2);
}

fn count(grid: &Vec<String>, right: usize, down: usize) -> usize {
    let width = grid.get(0).unwrap().len();
    let mut col = 0;
    let mut trees = 0;

    for line in grid.iter().step_by(down) {
        if line.chars().nth(col).unwrap() == '#' {
            trees += 1;
        }
        col = (col + right) % width;
    }

    trees
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
        assert_eq!(
            slopes
                .iter()
                .map(|(right, down)| count(&v, *right, *down))
                .fold(1, |prod, x| prod * x),
            336
        );
    }
}
