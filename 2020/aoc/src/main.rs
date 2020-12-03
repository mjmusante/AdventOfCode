use std::time::Instant;

mod day01;
mod day02;
mod day03;

type Func = fn();

fn main() {
    let runners: Vec<Func> = vec![day01::run, day02::run, day03::run];

    for run in runners {
        let now = Instant::now();
        run();
        let elapsed = now.elapsed().as_millis();
        println!(" -> Time: {}ms", elapsed);
    }
}
