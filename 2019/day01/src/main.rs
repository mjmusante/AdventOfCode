use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn fuel(mass : i64) -> i64 {
    mass / 3 - 2
}

fn account_for_fuel(fuelmass : i64) -> i64 {
    let mut fmass = fuelmass;
    let mut adjustment = 0;
    loop {
        fmass = fuel(fmass);
        if fmass <= 0 {
            break;
        }
        adjustment += fmass;
    }

    adjustment
}


fn main() {
    let masses = vec![12, 14, 1969, 100756];
    let fuels = vec![2, 2, 654, 33583];

    for i in 0..masses.len() {
        let m = masses[i];
        let f = fuels[i];
        let a = account_for_fuel(f);
        println!("mass {} -> fuel {} + adjustment {} = total {}", m, fuel(m), a, a + f);
        if fuel(m) != f {
            println!("\toopsie\n");
        }
    }

    let f = File::open("data/input.txt").unwrap();
    let vlist =  BufReader::new(&f)
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    let mut total = 0;
    let mut tplus = 0;
    for m in vlist.iter().map(|line| line.parse::<i64>().unwrap()) {
        let f = fuel(m);
        total += f;
        tplus += f + account_for_fuel(f);
    }

    println!("total fuel  = {}", total);
    println!("super total = {}", tplus);
}
