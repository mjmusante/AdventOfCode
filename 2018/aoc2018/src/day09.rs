use lines;

pub fn run() -> (String, String) {
    let lines = lines::lineread(String::from("puzzle_data/day09.txt"));

    (part1(&lines), part2(&lines))
}

fn part1(lines: &Vec<String>) -> String {
    lines.len().to_string()
}

fn part2(lines: &Vec<String>) -> String {
    lines.len().to_string()
}
