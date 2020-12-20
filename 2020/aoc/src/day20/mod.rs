use std::collections::{HashMap, HashSet};

use aoc::utils::lines;

pub fn run() {
    let lines = lines("data/20.txt");
    let m = parse(&lines);
    
    println!("Part 1 = {}", can_match(&m));
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Tile {
    num: i64,
    north: i64,
    east: i64,
    south: i64,
    west: i64,
    rot: i64,
    flipped: bool
}

impl Tile {
    pub fn fits_north(&self, alt: &Tile) -> bool {
        self.south == reverse(alt.north)
    }
    pub fn fits_south(&self, alt: &Tile) -> bool {
        self.north == reverse(alt.south)
    }
    pub fn fits_east(&self, alt: &Tile) -> bool {
        self.west == reverse(alt.east)
    }
    pub fn fits_west(&self, alt: &Tile) -> bool {
        self.east == reverse(alt.west)
    }
}

fn reverse(mut i: i64) -> i64 {
    let mut r = 0;
    for _ in 0..10 {
        r <<= 1;
        r |= i & 1;
        i >>= 1;
    }

    r
}

fn to_num(s: &String) -> i64 {
    let mut num = 0;
    for c in s.chars() {
        num <<= 1;
        if c == '#' {
            num |= 1;
        }
    }

    num
}

fn rotate_tile_right(t: &Tile) -> Tile {
    Tile {
        num: t.num,
        north: t.west,
        east: t.north,
        south: t.east,
        west: t.south,
        rot: t.rot + 1,
        flipped: t.flipped,
    }
}

fn parse(lines: &Vec<String>) -> Vec<Tile> {
    let mut result = Vec::new();
    let mut tile = Tile {
        num: 0,
        north: 0,
        east: 0,
        south: 0,
        west: 0,
        rot: 0,
        flipped: false,
    };
    let mut row = 0;

    for l in lines {
        if l == "" {
            result.push(tile);
            tile = Tile {
                num: 0,
                north: 0,
                east: 0,
                south: 0,
                west: 0,
                rot: 0,
                flipped: false,
            };
            continue;
        }
        if l.starts_with("Tile ") {
            tile.num = l[5..9].parse().expect("invalid number");
            row = 0;
            continue;
        }
        match row {
            0 => {
                tile.north = to_num(l);
                tile.east = if &l[9..=9] == "#" { 1 } else { 0 };
                tile.west = if &l[0..=0] == "#" { 1 } else { 0 };
            }
            1..=8 => {
                tile.east <<= 1;
                tile.east |= if &l[9..=9] == "#" { 1 } else { 0 };
                tile.west |= if &l[0..=0] == "#" { 1 << row } else { 0 };
            }
            9 => {
                tile.south = reverse(to_num(l));
                tile.east <<= 1;
                tile.east |= if &l[9..=9] == "#" { 1 } else { 0 };
                tile.west |= if &l[0..=0] == "#" { 1 << row } else { 0 };
            }
            x => {
                panic!(format!("too many rows {}", x));
            }
        }
        row += 1;
    }
    result.push(tile);

    result
}

fn flip(tile: &Tile) -> Tile {
    Tile {
        num: tile.num,
        north: reverse(tile.south),
        east: reverse(tile.east),
        south: reverse(tile.north),
        west: reverse(tile.west),
        rot: tile.rot,
        flipped: !tile.flipped,
    }
}

fn try_fit(map: &HashMap::<(i64, i64), &Tile>, pos: &(i64, i64), t: &Tile) -> Vec<Tile> {
    let mut result = Vec::new();
    let north = &(pos.0, pos.1+1);
    let south = &(pos.0, pos.1-1);
    let east = &(pos.0+1, pos.1);
    let west = &(pos.0-1, pos.1);

    let mut cur_t : Tile = *t;

    for flipped in &[false, true] {
        if *flipped {
            cur_t = flip(&cur_t);
        }
        for _ in 0..4 {
            if let Some(val) = map.get(north) {
                if cur_t.fits_south(val) {
                    result.push(cur_t)
                }
            }
            if let Some(val) = map.get(south) {
                if cur_t.fits_north(val) {
                    result.push(cur_t)
                }
            }
            if let Some(val) = map.get(east) {
                if cur_t.fits_west(val) {
                    result.push(cur_t)
                }
            }
            if let Some(val) = map.get(west) {
                if cur_t.fits_east(val) {
                    result.push(cur_t)
                }
            }
            cur_t = rotate_tile_right(&cur_t);
        }
    }

    result
}

fn solve(map: &HashMap::<(i64, i64), &Tile>, hs: &HashSet<&Tile>) -> i64 {
    if hs.is_empty() {
        let mut min = (0, 0);
        let mut max = (0, 0);
        for i in map.keys() {
            if i.0 < min.0 {
                min.0 = i.0;
            }
            if i.1 < min.1 {
                min.1 = i.1;
            }
            if i.0 > max.0 {
                max.0 = i.0;
            }
            if i.1 > max.1 {
                max.1 = i.1;
            }
        }
        // println!("bounds = [{:?}, {:?}]", min, max);
        // for i in min.0..=max.0 {
        //     for j in min.1..=max.1 {
        //         print!(" {}", map.get(&(i, j)).unwrap().num);
        //     }
        //     println!("");
        // }
        return
            map.get(&(min.0, min.1)).unwrap().num *
            map.get(&(min.0, max.1)).unwrap().num *
            map.get(&(max.0, min.1)).unwrap().num *
            map.get(&(max.0, max.1)).unwrap().num;
    }

    for tile in hs {
        for k in map.keys() {
            for i in -1..=1 {
                for j in -1..=1 {
                    let loc = &(k.0 + i, k.1 + j);
                    if (i == 0 && j == 0) || map.contains_key(loc) {
                        continue;
                    }
                    for t in try_fit(map, loc, tile) {
                        let mut newmap = map.clone();
                        newmap.insert(*loc, &t);
                        let mut remainder = hs.clone();
                        remainder.remove(tile);
                        let result = solve(&newmap, &remainder);
                        if result != 0 {
                            return result;
                        }
                    }
                }
            }
        }
    }

    0
}

fn can_match(tiles: &Vec<Tile>) -> i64 {
    let hs : HashSet<&Tile> = tiles.iter().collect();

    for tile in &hs {
        let mut h2 : HashSet<&Tile> = HashSet::new();
        h2.insert(tile);
        let remainder : HashSet<&Tile> = hs.difference(&h2).map(|x| *x).collect();
        let mut map = HashMap::<(i64, i64), &Tile>::new();
        map.insert((0, 0), tile);
        let answer = solve(&map, &remainder);
        if answer != 0 {
            return answer;
        }
    }

    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn testx() {
        assert_eq!(to_num(&"..##.#..#.".to_string()), 210);
        assert_eq!(reverse(231), 924);
    }

    #[test]
    fn test1() {
        let v = vec![
            "Tile 2311:".to_string(),
            "..##.#..#.".to_string(),
            "##..#.....".to_string(),
            "#...##..#.".to_string(),
            "####.#...#".to_string(),
            "##.##.###.".to_string(),
            "##...#.###".to_string(),
            ".#.#.#..##".to_string(),
            "..#....#..".to_string(),
            "###...#.#.".to_string(),
            "..###..###".to_string(),
            "".to_string(),
            "Tile 1951:".to_string(),
            "#.##...##.".to_string(),
            "#.####...#".to_string(),
            ".....#..##".to_string(),
            "#...######".to_string(),
            ".##.#....#".to_string(),
            ".###.#####".to_string(),
            "###.##.##.".to_string(),
            ".###....#.".to_string(),
            "..#.#..#.#".to_string(),
            "#...##.#..".to_string(),
            "".to_string(),
            "Tile 1171:".to_string(),
            "####...##.".to_string(),
            "#..##.#..#".to_string(),
            "##.#..#.#.".to_string(),
            ".###.####.".to_string(),
            "..###.####".to_string(),
            ".##....##.".to_string(),
            ".#...####.".to_string(),
            "#.##.####.".to_string(),
            "####..#...".to_string(),
            ".....##...".to_string(),
            "".to_string(),
            "Tile 1427:".to_string(),
            "###.##.#..".to_string(),
            ".#..#.##..".to_string(),
            ".#.##.#..#".to_string(),
            "#.#.#.##.#".to_string(),
            "....#...##".to_string(),
            "...##..##.".to_string(),
            "...#.#####".to_string(),
            ".#.####.#.".to_string(),
            "..#..###.#".to_string(),
            "..##.#..#.".to_string(),
            "".to_string(),
            "Tile 1489:".to_string(),
            "##.#.#....".to_string(),
            "..##...#..".to_string(),
            ".##..##...".to_string(),
            "..#...#...".to_string(),
            "#####...#.".to_string(),
            "#..#.#.#.#".to_string(),
            "...#.#.#..".to_string(),
            "##.#...##.".to_string(),
            "..##.##.##".to_string(),
            "###.##.#..".to_string(),
            "".to_string(),
            "Tile 2473:".to_string(),
            "#....####.".to_string(),
            "#..#.##...".to_string(),
            "#.##..#...".to_string(),
            "######.#.#".to_string(),
            ".#...#.#.#".to_string(),
            ".#########".to_string(),
            ".###.#..#.".to_string(),
            "########.#".to_string(),
            "##...##.#.".to_string(),
            "..###.#.#.".to_string(),
            "".to_string(),
            "Tile 2971:".to_string(),
            "..#.#....#".to_string(),
            "#...###...".to_string(),
            "#.#.###...".to_string(),
            "##.##..#..".to_string(),
            ".#####..##".to_string(),
            ".#..####.#".to_string(),
            "#..#.#..#.".to_string(),
            "..####.###".to_string(),
            "..#.#.###.".to_string(),
            "...#.#.#.#".to_string(),
            "".to_string(),
            "Tile 2729:".to_string(),
            "...#.#.#.#".to_string(),
            "####.#....".to_string(),
            "..#.#.....".to_string(),
            "....#..#.#".to_string(),
            ".##..##.#.".to_string(),
            ".#.####...".to_string(),
            "####.#.#..".to_string(),
            "##.####...".to_string(),
            "##..#.##..".to_string(),
            "#.##...##.".to_string(),
            "".to_string(),
            "Tile 3079:".to_string(),
            "#.#.#####.".to_string(),
            ".#..######".to_string(),
            "..#.......".to_string(),
            "######....".to_string(),
            "####.#..#.".to_string(),
            ".#...#.##.".to_string(),
            "#.#####.##".to_string(),
            "..#.###...".to_string(),
            "..#.......".to_string(),
            "..#.###...".to_string(),
        ];
        let m = parse(&v);

        assert_eq!(can_match(&m), 20899048083289);
    }
}
