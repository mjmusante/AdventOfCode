use std::collections::HashSet;

use aoc::utils::lines;

pub fn run() {
    let data = lines("data/24.txt");

    println!("Part 1 = {}", flip_and_count(&data));
}

// east: x + 1, y
// west: x - 1, y
// southeast: x + 1 if y is odd, y + 1
// southwest: x - 1 if y is even, y + 1
// northeast: x + 1 if y is odd, y - 1,
// northwest: x - 1 if y is even, y - 1
// enum HexDir {
//     East, SouthEast, SouthWest, West, NorthWest, NorthEast
// }

// use HexDir::*;

fn flip_and_count(list: &Vec<String>) -> i64 {
    let mut floor = HashSet::new();

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
    }
    floor.len() as i64
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let v = vec![
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
        ];

        assert_eq!(flip_and_count(&v), 10);
    }

    #[test]
    fn test2() {
        let v = vec![
            "senwswne".to_string(),           // should be (0, 0) (turn black)
            "eeee".to_string(),               // should be (4, 0) (turn black)
            "sesesese".to_string(),           // should be (2, 4) (turn black)
            "eeeeswswswsw".to_string(),       // should be (2, 4) (turn white)
            "eeesesesese".to_string(),        // should be (5, 4) (turn black)
            "sesesesenenenene".to_string(),   // should be (4, 0) (turn white)
        ];
        assert_eq!(flip_and_count(&v), 2);
    }
}
