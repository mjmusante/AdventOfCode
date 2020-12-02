use std::time::Instant;

mod day01;
mod day02;

fn main() {
    let now = Instant::now();
    day01::day01();
    let elapsed = now.elapsed().as_millis();
    println!(" -> Time: {}ms", elapsed);

    let now = Instant::now();
    day02::day02();
    let elapsed = now.elapsed().as_millis();
    println!(" -> Time: {}ms", elapsed);
}
