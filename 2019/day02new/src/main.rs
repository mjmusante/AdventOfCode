use intcode::Computer;

fn main() {
    let mut c = Computer::new().from_file("inputs/day02.txt");

    c.set_noun_verb(12, 2);
    c.run();
    let part1 = c.peek(0);

    let mut part2 = -1;
    'outer: for i in 0..=99 {
        for j in 0..=99 {
            c.reset();
            c.set_noun_verb(i, j);
            c.run();
            if c.peek(0) == 19690720 {
                part2 = 100 * i + j;
                break 'outer;
            }
        }
    }

    println!("part 1 = {}", part1);
    println!("part 2 = {}", part2);
}
