use intcode::Computer;

struct Nic {
    c: Computer,
    q: Vec<(i64, i64)>,
    f: bool,
}

fn main() {
    let mut nic = vec![];

    for i in 0..50 {
        nic.push(Nic {
            c: Computer::new().from_file("inputs/day23.txt"),
            q: Vec::new(),
            f: false,
        });
        nic[i].c.run_with_input(i as i64);
    }

    'outer: loop {
        let mut did_work = false;
        for i in 0..50 {
            if nic[i].c.waiting_for_input() {
                if !nic[i].f {
                    if nic[i].q.len() > 0 {
                        let v = nic[i].q[0].0;
                        nic[i].c.run_with_input(v);
                        nic[i].f = true;
                    } else {
                        nic[i].c.run_with_input(-1);
                    }
                } else {
                    if nic[i].q.len() > 0 {
                        let v = nic[i].q[0].1;
                        nic[i].c.run_with_input(v);
                        nic[i].q.remove(0);
                        nic[i].f = false;
                    } else {
                        nic[i].c.run_with_input(-1);
                    }
                }
                did_work = true;
            }

            if nic[i].c.has_output() {
                let dest = nic[i].c.next_output() as usize;
                let x = nic[i].c.next_output();
                let y = nic[i].c.next_output();
                if dest == 255 {
                    println!("part 1 = {}", y);
                    break 'outer;
                }
                nic[dest].q.push((x, y));
                did_work = true;
            }
        }
        if !did_work {
            println!("loop completed without doing work!");
            break;
        }
    }
}
