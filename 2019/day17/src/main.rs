use intcode::Computer;

fn main() {
    let mut c = Computer::new().from_file("inputs/day17.txt");
    c.run_with_input(42);
    println!("part 1 = {}", c.next_output());
}
