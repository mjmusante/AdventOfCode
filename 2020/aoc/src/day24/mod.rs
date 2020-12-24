use std::collections::HashSet;

use aoc::utils::lines;

pub fn run() {
    let data = lines("data/24.txt");
    let ans = flip_and_count(&data);

    println!("Part 1 = {}", ans.0);
    println!("Part 2 = {}", ans.1);
}

fn flip_and_count(list: &Vec<String>) -> (i64, i64) {
    let mut floor = HashSet::new();
    let mut min_x = 0;
    let mut min_y = 0;
    let mut max_x = 0;
    let mut max_y = 0;

    for i in list {
        let mut x = 0;
        let mut y = 0;
        let mut last_dir = 'x';
        for c in i.chars() {
            match c {
                's' => {
                    y += 1;
                    last_dir = 's';
                }
                'n' => {
                    y -= 1;
                    last_dir = 'n';
                }
                'e' => {
                    match last_dir {
                        's' => {
                            if y & 1 == 0 {
                                x += 1
                            }
                        }
                        'n' => {
                            if y & 1 == 0 {
                                x += 1
                            }
                        }
                        'x' => x += 1,
                        _ => {
                            panic!("bug");
                        }
                    };
                    last_dir = 'x';
                }
                'w' => {
                    match last_dir {
                        's' => {
                            if y & 1 == 1 {
                                x -= 1
                            }
                        }
                        'n' => {
                            if y & 1 == 1 {
                                x -= 1
                            }
                        }
                        'x' => x -= 1,
                        _ => {
                            panic!("bug");
                        }
                    };
                    last_dir = 'x';
                }
                _ => {
                    panic!("invalid input");
                }
            };
            // println!("step {}: ({}, {})", c, x, y);
        }
        // println!("flipping ({}, {})", x, y);
        if floor.contains(&(x, y)) {
            floor.remove(&(x, y));
        } else {
            floor.insert((x, y));
        }
        if max_x < x {
            max_x = x;
        }
        if min_x > x {
            min_x = x;
        }
        if max_y < y {
            max_y = y;
        }
        if min_y > y {
            min_y = y;
        }
    }

    let part1 = floor.len() as i64;

    // east, southeast, southwest, west, northwest, northeast
    let odd_y = vec![(1, 0), (1, 1), (0, 1), (-1, 0), (0, -1), (1, -1)];
    let even_y = vec![(1, 0), (0, 1), (-1, 1), (-1, 0), (-1, -1), (0, -1)];

    // now play life
    for i in 0..100 {
        let mut new_floor = HashSet::new();
        min_x -= 1; // we'll search an area that's
        min_y -= 1; // bigger than we need to, but
        max_x += 1; // it's good enough for AoC
        max_y += 1; // Burma Shave

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                let search = if y & 1 == 0 { &even_y } else { &odd_y };
                let mut count = 0;
                for s in search {
                    if floor.contains(&(x + s.0, y + s.1)) {
                        count += 1;
                    }
                }
                if floor.contains(&(x, y)) {
                    if count == 1 || count == 2 {
                        // stays black
                        new_floor.insert((x, y));
                    }
                } else {
                    if count == 2 {
                        // turns black
                        new_floor.insert((x, y));
                    }
                }
            }
        }

        floor = new_floor;
    }

    (part1, floor.len() as i64)
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_data() -> Vec<String> {
        vec![
            "sesenwnenenewseeswwswswwnenewsewsw".to_string(),
            "neeenesenwnwwswnenewnwwsewnenwseswesw".to_string(),
            "seswneswswsenwwnwse".to_string(),
            "nwnwneseeswswnenewneswwnewseswneseene".to_string(),
            "swweswneswnenwsewnwneneseenw".to_string(),
            "eesenwseswswnenwswnwnwsewwnwsene".to_string(),
            "sewnenenenesenwsewnenwwwse".to_string(),
            "wenwwweseeeweswwwnwwe".to_string(),
            "wsweesenenewnwwnwsenewsenwwsesesenwne".to_string(),
            "neeswseenwwswnwswswnw".to_string(),
            "nenwswwsewswnenenewsenwsenwnesesenew".to_string(),
            "enewnwewneswsewnwswenweswnenwsenwsw".to_string(),
            "sweneswneswneneenwnewenewwneswswnese".to_string(),
            "swwesenesewenwneswnwwneseswwne".to_string(),
            "enesenwswwswneneswsenwnewswseenwsese".to_string(),
            "wnwnesenesenenwwnenwsewesewsesesew".to_string(),
            "nenewswnwewswnenesenwnesewesw".to_string(),
            "eneswnwswnwsenenwnwnwwseeswneewsenese".to_string(),
            "neswnwewnwnwseenwseesewsenwsweewe".to_string(),
            "wseweeenwnesenwwwswnew".to_string(),
        ]
    }

    #[test]
    fn test1() {
        let v = test_data();
        let ans = flip_and_count(&v);

        assert_eq!(ans.0, 10);
        assert_eq!(ans.1, 2208);
    }

    #[test]
    fn test2() {
        let v = vec![
            "senwswne".to_string(),         // should be (0, 0) (turn black)
            "eeee".to_string(),             // should be (4, 0) (turn black)
            "sesesese".to_string(),         // should be (2, 4) (turn black)
            "eeeeswswswsw".to_string(),     // should be (2, 4) (turn white)
            "eeesesesese".to_string(),      // should be (5, 4) (turn black)
            "sesesesenenenene".to_string(), // should be (4, 0) (turn white)
        ];
        assert_eq!(flip_and_count(&v).0, 2);
    }
}
