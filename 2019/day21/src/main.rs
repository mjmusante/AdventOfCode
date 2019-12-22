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

    let part1i = [
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

    c.run();
    while c.has_output() {
        print!("{}", c.next_output() as u8 as char);
    }

    for i in part1i.iter() {
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

    c.reset();

    /*
     * @
     * #ABCDEFGHI#
     *  111
     *     11  1
     *
     */

    let part2i = [
        "OR A J", "AND B J", "AND C J", "NOT J J", "AND D J", "OR E T", "OR H T", "AND T J", "RUN",
    ];

    c.run();
    while c.has_output() {
        print!("{}", c.next_output() as u8 as char);
    }

    for i in part2i.iter() {
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
            println!("part 2 = {}", ch);
        }
    }
}
