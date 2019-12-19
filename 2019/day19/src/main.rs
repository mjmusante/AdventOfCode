use intcode::Computer;

fn in_beam(c: &mut Computer, x: i64, y: i64) -> bool {
    c.reset();
    c.run_with_input(x);
    c.run_with_input(y);
    c.next_output() == 1
}

fn main() {
    let mut c = Computer::new().from_file("inputs/day19.txt");

    let mut count = 0;
    'outer: for x in 0..50 {
        for y in 0..50 {
            if in_beam(&mut c, x, y) {
                count += 1;
            }
        }
    }
    println!("part 1 = {}", count);

    let mut startx = 78;
    let mut starty = 91;

    assert!(in_beam(&mut c, startx, starty));
    assert!(!in_beam(&mut c, startx + 1, starty));

    while startx < 100 {
        starty += 1;
        while in_beam(&mut c, startx + 1, starty) {
            startx += 1;
        }
    }
    while !in_beam(&mut c, startx - 99, starty) || !in_beam(&mut c, startx - 99, starty + 99) {
        starty += 1;
        while in_beam(&mut c, startx + 1, starty) {
            startx += 1;
        }
    }

    println!("part 2 = {}", 10_000 * (startx - 99) + starty);
}
