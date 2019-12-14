use std::process::exit;

use intcode::Computer;

fn main() {
    let part1;
    let mut c = Computer::new().from_file("inputs/day05.txt");
    c.run_with_input(1);
    assert!(c.halted());
    loop {
        if !c.has_output() {
            println!("Computer[1] failed to generate output");
            exit(1);
        }
        let val = c.next_output();
        if val != 0 {
            part1 = val;
            break;
        }
    }

    c.reset();
    c.run_with_input(5);
    if !c.has_output() {
        println!("Computer[5] failed to generate output");
        exit(1);
    }
    assert!(c.halted());
    let part2 = c.next_output();

    println!("part 1 = {}", part1);
    println!("part 2 = {}", part2);
}
