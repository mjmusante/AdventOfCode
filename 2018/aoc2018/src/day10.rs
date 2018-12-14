use lines;
use regex::Regex;
use std::cmp::{max, min};

pub fn run() -> (String, String) {
    let lines = lines::lineread("puzzle_data/day10.txt".to_string());
    let (part1, part2) = solve(&lines, 20000);
    let mut sky = String::from("\n");
    for p in part1 {
        sky.push_str(&p);
        sky.push('\n');
    }
    (sky, part2.to_string())
}

struct Point {
    x: i64,
    y: i64,
    dx: i64,
    dy: i64
}

fn solve(v: &Vec<String>, secs: u64) -> (Vec<String>, u64) {
    const SIZEX : usize = 64;
    const SIZEY : usize = 10;
    let mut stars : Vec<Point> = vec![];
    // let reg = Regex::new(r"position=<([ -]?\d+), ([ -]?\d+)> velocity=<([ -]?\d+), ([ -]?\d+)>").unwrap();
    let reg = Regex::new(r"position=<[ ]?([-]?[0-9]+), [ ]?([-]?[0-9]+)> velocity=<[ ]?([-]?[0-9]+), [ ]?([-]?[0-9]+)>").unwrap();
    for line in v {
        let foo = reg.captures_iter(line).next().unwrap();
        let x = foo[1].parse::<i64>().unwrap();
        let y = foo[2].parse::<i64>().unwrap();
        let dx = foo[3].parse::<i64>().unwrap();
        let dy = foo[4].parse::<i64>().unwrap();
        stars.push( Point { x: x, y: y, dx: dx, dy: dy} );
    }

    for s in 0..secs {
        let mut disp = [['.'; SIZEY]; SIZEX];
        // let mut show_page = false;
        let mut maxx = stars[0].x;
        let mut maxy = stars[0].y;
        let mut minx = maxx;
        let mut miny = maxy;

        for p in &mut stars {
            p.x += p.dx;
            p.y += p.dy;
            minx = min(p.x, minx);
            miny = min(p.y, miny);
            maxx = max(p.x, maxx);
            maxy = max(p.y, maxy);
            // if p.x < 0 || p.y < 0 || p.x >= SIZEX as i64 || p.y >= SIZEY as i64 {
            //     continue;
            // }
            // disp[p.x as usize][p.y as usize] = '#';
            // show_page = true;
        }

        if secs == 3 && s < 2 {
            continue;
        }
        if ((maxx - minx) as usize) < SIZEX && ((maxy - miny) as usize) < SIZEY {
            let mut ans = vec![];
            for p in &stars {
                disp[(p.x - minx) as usize][(p.y - miny) as usize] = '#';
            }

            for y in 0..SIZEY {
                let mut l = String::from("");
                for x in 0..SIZEX {
                    l.push(disp[x][y]);
                }
                ans.push(l);
            }

            return (ans, s + 1);
        }
    }
    (vec![], 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vec_of_strings {
        ($($x:expr),*) => (vec![$($x.to_string()),*]);
    }

    #[test]
    fn day10_test1() {
        let v = vec_of_strings![
            "position=< 9,  1> velocity=< 0,  2>",
            "position=< 7,  0> velocity=<-1,  0>",
            "position=< 3, -2> velocity=<-1,  1>",
            "position=< 6, 10> velocity=<-2, -1>",
            "position=< 2, -4> velocity=< 2,  2>",
            "position=< 1,  8> velocity=< 1, -1>",
            "position=<-6, 10> velocity=< 2, -2>",
            "position=< 1,  7> velocity=< 1,  0>",
            "position=<-3, 11> velocity=< 1, -2>",
            "position=< 7,  6> velocity=<-1, -1>",
            "position=<-2,  3> velocity=< 1,  0>",
            "position=<-4,  3> velocity=< 2,  0>",
            "position=<10, -3> velocity=<-1,  1>",
            "position=< 5, 11> velocity=< 1, -2>",
            "position=< 4,  7> velocity=< 0, -1>",
            "position=< 8, -2> velocity=< 0,  1>",
            "position=<15,  0> velocity=<-2,  0>",
            "position=< 1,  6> velocity=< 1,  0>",
            "position=< 8,  9> velocity=< 0, -1>",
            "position=< 3,  3> velocity=<-1,  1>",
            "position=< 0,  5> velocity=< 0, -1>",
            "position=<-2,  2> velocity=< 2,  0>",
            "position=< 5, -2> velocity=< 1,  2>",
            "position=< 1,  4> velocity=< 2,  1>",
            "position=<-2,  7> velocity=< 2, -2>",
            "position=< 3,  6> velocity=<-1, -1>",
            "position=< 5,  0> velocity=< 1,  0>",
            "position=<-6,  0> velocity=< 2,  0>",
            "position=< 5,  9> velocity=< 1, -2>",
            "position=<14,  7> velocity=<-2,  0>",
            "position=<-3,  6> velocity=< 2, -1>"
       ];
       let result = vec_of_strings![
            "#...#..###......................................................",
            "#...#...#.......................................................",
            "#...#...#.......................................................",
            "#####...#.......................................................",
            "#...#...#.......................................................",
            "#...#...#.......................................................",
            "#...#...#.......................................................",
            "#...#..###......................................................",
            "................................................................",
            "................................................................"
       ];
       let (x, y) = solve(&v, 3);
       assert_eq!(x, result);
       assert_eq!(y, 3);
    }
}
