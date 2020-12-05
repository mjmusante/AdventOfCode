use aoc::utils::lines;

pub fn run() {
    let lines = lines("data/05.txt");

    let mut list = lines.iter().map(convert).collect::<Vec<i64>>();
    list.sort();

    println!("Part 1 = {}", list.get(list.len() - 1).unwrap());

    let mut last = 0;
    for seat in list {
        if last + 2 == seat {
            println!("Part 2 = {}?", seat - 1);
            break; // assuming AoC's input only has one match
        }
        last = seat;
    }
}

fn convert(id: &String) -> i64 {
    i64::from_str_radix(
        id.chars()
            .map(|c| match c {
                'B' | 'R' => '1',
                'F' | 'L' => '0',
                _ => {
                    panic!("invalid character in id");
                }
            })
            .collect::<String>()
            .as_str(),
        2,
    )
    .unwrap()
}
