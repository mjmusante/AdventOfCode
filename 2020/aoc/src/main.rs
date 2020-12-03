use core::time::Duration;
use std::time::Instant;

mod day01;
mod day02;
mod day03;

type Func = fn();

fn main() {
    let runners: Vec<Func> = vec![day01::run, day02::run, day03::run];
    let mut total = Duration::new(0, 0);

    for run in runners {
        let now = Instant::now();
        run();
        let elapsed = now.elapsed();

        total += elapsed;
        println!(" -> Time: {}ms", elapsed.as_millis());
    }
    println!("\nTotal time: {}ms", total.as_millis());
}
