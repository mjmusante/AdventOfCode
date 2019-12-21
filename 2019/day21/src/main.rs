use intcode::Computer;

/*

@
#....# -> dead

@
#...## -> jump

@
#..#.# -> too late

@
#..### -> jump

@
#.#..# -> too late

@
#.#.## -> jump

@
#.##.# -> too late

@
#.#### -> jump

@
##...# -> step forward

@
##..## -> jump

@
##.#.# -> step forward

@
##.### -> jump

@
###..# -> step forward

@
###.## -> step forward

@
####.# -> step forward

@
###### -> step forward

    Truth Table

    A B C D   J
    --------+--
    . . . . | X
    . . . 1 | 1 (3)
    . . 1 . | X
    . . 1 1 | 1 (3)
    . 1 . . | X
    . 1 . 1 | 1 (2)
    . 1 1 . | X
    . 1 1 1 | 1 (2)
    1 . . . | 0
    1 . . 1 | 1 (1)
    1 . 1 . | 0
    1 . 1 1 | 1 (1)
    1 1 . . | 0
    1 1 . 1 | 1 (?)
    1 1 1 . | 0
    1 1 1 1 | 0

    1 = (A and NOT B and D) +
    2 = (not A and B and D) +
    3 = (not A and not B and D)

*/

fn main() {
    let mut c = Computer::new().from_file("inputs/day21.txt");

    c.run();
    while c.has_output() {
        print!("{}", c.next_output() as u8 as char);
    }

    let instructions = [
        "NOT A T", "AND D T", "OR T J", "NOT B T", "AND A T", "AND D T", "OR T J", "NOT C T",
        "AND D T", "AND A T", "OR T J", "WALK",
    ];

    for i in instructions.iter() {
        println!("> {}", i);
        for code in i.chars() {
            c.run_with_input(code as i64);
        }
        c.run_with_input(10);
    }

    while c.has_output() {
        let ch = c.next_output();
        if ch < 256 {
            print!("{}", ch as u8 as char);
        } else {
            println!("part 1 = {}", ch);
        }
    }
}
