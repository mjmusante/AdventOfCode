use std::process::exit;

use core::cmp::max;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn cast_ray(
    aster: &mut Vec<Vec<char>>,
    row: usize,
    col: usize,
    delta_r: isize,
    delta_c: isize,
) -> i64 {
    let width = aster[0].len() as isize;
    let height = aster.len() as isize;
    let mut r = row as isize + delta_r;
    let mut c = col as isize + delta_c;

    let mut count = 0;
    let mut found = false;
    while r >= 0 && c >= 0 && r < height && c < width {
        let ru = r as usize;
        let cu = c as usize;
        if found {
            aster[ru][cu] = 'x';
        } else if aster[ru][cu] == '#' {
            count += 1;
            aster[ru][cu] = '*';
            found = true;
        } else if aster[ru][cu] == '.' {
            aster[ru][cu] = '_';
        }

        r += delta_r;
        c += delta_c;
    }
    count
}

fn count_visible(mut aster: &mut Vec<Vec<char>>, row: usize, col: usize) -> i64 {
    let width = aster[0].len();
    let height = aster.len();
    let mut count = 0;

    for i in 1..=max(width, height) {
        let min_x = if col > i - 1 { col - i } else { 0 };
        let min_y = if row > i - 1 { row - i } else { 0 };
        let max_x = if col + i < width - 1 {
            col + i
        } else {
            width - 1
        };
        let max_y = if row + i < height - 1 {
            row + i
        } else {
            height - 1
        };

        let left_stride = (col - min_x) as isize;
        let right_stride = (max_x - col) as isize;
        let up_stride = (row - min_y) as isize;
        let down_stride = (max_y - row) as isize;

        for c in -left_stride..=right_stride {
            if up_stride > 0 {
                count += cast_ray(&mut aster, row, col, -up_stride, c);
            }
            if down_stride > 0 {
                count += cast_ray(&mut aster, row, col, down_stride, c);
            }
        }

        for r in -up_stride..=down_stride {
            if left_stride > 0 {
                count += cast_ray(&mut aster, row, col, r, -left_stride);
            }
            if right_stride > 0 {
                count += cast_ray(&mut aster, row, col, r, right_stride);
            }
        }
    }

    count
}

fn compute_angle(start_y: usize, start_x: usize, end_y: usize, end_x: usize) -> f64 {
    let sy = start_y as f64;
    let sx = start_x as f64;
    let ey = end_y as f64;
    let ex = end_x as f64;
    (ex - sx).atan2(ey - sy)
}

fn main() {
    let f = File::open("inputs/day10.txt").unwrap();
    let vlist = BufReader::new(&f)
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    let mut aster: Vec<Vec<char>> = vec![];
    for v in vlist {
        aster.push(v.chars().collect());
    }

    let width = aster[0].len();
    let height = aster.len();
    let mut monitor_row = 0;
    let mut monitor_col = 0;
    let mut endmap = vec![];

    let mut count = 0;
    for row in 0..height {
        for col in 0..width {
            if aster[row][col] == '#' {
                let mut a = aster.clone();
                let c = count_visible(&mut a, row, col);
                if c > count {
                    count = c;
                    monitor_row = row;
                    monitor_col = col;
                    endmap = a.clone();
                }
            }
        }
    }
    println!(
        "part 1 = {}, coords = ({},{})",
        count, monitor_col, monitor_row
    );

    if count < 200 {
        println!("Multiple-sweeps not implemented yet");
        exit(1);
    }

    let mut angle_list = vec![];
    for row in 0..height {
        for col in 0..width {
            if endmap[row][col] == '*' {
                angle_list.push((row, col, compute_angle(monitor_row, monitor_col, row, col)));
            }
        }
    }

    angle_list.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
    println!("part 2 = {}", angle_list[199].1 * 100 + angle_list[199].0);
}
