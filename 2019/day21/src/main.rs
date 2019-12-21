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
    . . . 1 | 1 (1)
    . . 1 . | X
    . . 1 1 | 1 (1)
    . 1 . . | X
    . 1 . 1 | 1 (1)
    . 1 1 . | X
    . 1 1 1 | 1 (1)
    1 . . . | 0
    1 . . 1 | 1 (2) (!B && D)
    1 . 1 . | 0
    1 . 1 1 | 1 (2) (!B && D)
    1 1 . . | 0
    1 1 . 1 | 1 (3) (!C && D)
    1 1 1 . | 0
    1 1 1 1 | 0

*/

fn main() {
    let mut c = Computer::new().from_file("inputs/day21.txt");

    c.run();
    while c.has_output() {
        print!("{}", c.next_output() as u8 as char);
    }

    let instructions = [
        // case 1:
        "NOT A J", // if the next step is a hole, jump
        // case 2:
        "NOT B T", // if b is a hole
        "AND D T", // and we can jump
        "OR T J",  // then jump
        // case 3
        "NOT C T", // if C is a hole
        "AND D T", // and we can jump
        "OR T J",  // then jump
        "WALK",
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
