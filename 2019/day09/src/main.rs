use intcode::Computer;

fn main() {
    let mut c = Computer::new().from_file("inputs/day09.txt");
    for i in 1..=2 {
        c.reset();
        c.run_with_input(i);
        println!("part {} = {}", i, c.next_output());
    }
}
