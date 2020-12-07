use intcode::Computer;

use std::collections::HashSet;

struct Nic {
    c: Computer,
    q: Vec<(i64, i64)>,
    f: bool,
}

fn main() {
    let mut nic = vec![];
    let mut nat = (0, 0);
    let mut part1shown = false;
    let mut hs: HashSet<i64> = HashSet::new();

    for i in 0..50 {
        nic.push(Nic {
            c: Computer::new().from_file("inputs/day23.txt"),
            q: Vec::new(),
            f: false,
        });
        nic[i].c.run_with_input(i as i64);
    }

    loop {
        let mut did_work = false;
        for i in 0..50 {
            if nic[i].c.waiting_for_input() {
                if !nic[i].f {
                    if nic[i].q.len() > 0 {
                        let v = nic[i].q[0].0;
                        nic[i].c.run_with_input(v);
                        nic[i].f = true;
                        did_work = true;
                    } else {
                        nic[i].c.run_with_input(-1);
                    }
                } else {
                    if nic[i].q.len() > 0 {
                        let v = nic[i].q[0].1;
                        nic[i].c.run_with_input(v);
                        nic[i].q.remove(0);
                        nic[i].f = false;
                        did_work = true;
                    } else {
                        nic[i].c.run_with_input(-1);
                    }
                }
            }

            if nic[i].c.has_output() {
                let dest = nic[i].c.next_output() as usize;
                let x = nic[i].c.next_output();
                let y = nic[i].c.next_output();
                if dest == 255 {
                    if !part1shown {
                        println!("part 1 = {}", y);
                        part1shown = true;
                    }
                    nat = (x, y);
                } else {
                    nic[dest].q.push((x, y));
                }
                did_work = true;
            }
        }
        if !did_work {
            if hs.contains(&nat.1) {
                println!("part 2 = {}", nat.1);
                break;
            }
            hs.insert(nat.1);
            nic[0].q.push(nat);
        }
    }
}
