use aoc::utils::lines;

pub fn run() {
    let lines = lines("data/03.txt");

    let part1 = count(&lines, 3, 1);
    println!("Part 1 = {}", part1);

    let slopes = vec![(1, 1), (5, 1), (7, 1), (1, 2)];
    let part2 = slopes
        .iter()
        .map(|(row, col)| count(&lines, *row, *col))
        .fold(part1, |prod, x| prod * x);
    println!("Part 2 = {}", part2);
}

fn count(grid: &Vec<String>, right: usize, down: usize) -> usize {
    let mut row = 0;
    let mut col = 0;
    let mut trees = 0;

    while row < grid.len() {
        if has_tree(&grid, row, col) {
            trees += 1;
        }
        row += down;
        col += right;
    }

    trees
}

fn has_tree(grid: &Vec<String>, row: usize, col: usize) -> bool {
    if row >= grid.len() {
        return false;
    }
    let real_col = col % grid.get(0).unwrap().len();

    grid.get(row).unwrap().chars().nth(real_col).unwrap() == '#'
}
