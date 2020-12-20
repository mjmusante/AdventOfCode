use std::collections::{HashMap, HashSet};

use aoc::utils::lines;

pub fn run() {
    let lines = lines("data/20.txt");
    let m = parse(&lines);
    let what = can_match(&m);

    println!("Part 1 = {}", what.0);
    println!("Part 2 = {}", what.1);
}

type ImgData = [u8; 8];

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Tile {
    num: i64,
    north: i64,
    east: i64,
    south: i64,
    west: i64,
    rot: i64,
    flipped: bool,
    img: ImgData,
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

fn rotate_hashimg_right(
    old_img: &HashSet<(i64, i64)>,
    min: &(i64, i64),
    max: &(i64, i64),
) -> HashSet<(i64, i64)> {
    let mut result = HashSet::new();
    for col in min.0..=max.0 {
        for row in min.1..=max.1 {
            let rot = (max.0 - (row - min.1), min.1 + (col - min.0));
            // println!("{:?} <- {:?}", (col, row), rot);
            if old_img.contains(&rot) {
                result.insert((col, row));
            }
        }
    }
    println!("rotated");
    result
}

fn flip_hashimg(
    old_img: &HashSet<(i64, i64)>,
    min: &(i64, i64),
    max: &(i64, i64),
) -> HashSet<(i64, i64)> {
    let mut result = HashSet::new();
    for col in min.0..=max.0 {
        for row in min.1..=max.1 {
            if old_img.contains(&(max.0 - col, row)) {
                result.insert((col, row));
            }
        }
    }
    result
}

fn rotate_img_right(old_img: &ImgData) -> ImgData {
    let mut img = [0; 8];

    for bit in 0..8 {
        let mut data = 0;
        for i in (0..8).rev() {
            data <<= 1;
            data |= if (old_img[i] & (1 << bit)) != 0 { 1 } else { 0 };
        }
        img[7 - bit] = data;
    }

    img
}

fn flip_image(old_img: &ImgData) -> ImgData {
    let mut img = [0; 8];

    for i in 0..8 {
        img[7 - i] = old_img[i];
    }
    img
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
        img: rotate_img_right(&t.img),
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
        img: [0; 8],
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
                img: [0; 8],
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
                tile.img[row - 1] = to_num(&l[1..9].to_string()) as u8;
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

fn image_data(
    map: &HashMap<(i64, i64), &Tile>,
    min: &(i64, i64),
    max: &(i64, i64),
) -> HashSet<(i64, i64)> {
    let mut image = HashSet::new();

    for row in min.1..=max.1 {
        for col in min.0..=max.0 {
            let tile = map.get(&(col, row)).unwrap();
            for i in 0..8 {
                let d = tile.img[i];
                for bit in (0..8).rev() {
                    if (d & (1 << bit)) != 0 {
                        let locx = col * 8 + (7 - bit);
                        let locy = row * 8 + (7 - i as i64);
                        image.insert((locx, locy));
                    }
                }
            }
        }
    }

    image
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
        img: flip_image(&tile.img),
    }
}

fn try_fit(map: &HashMap<(i64, i64), &Tile>, pos: &(i64, i64), t: &Tile) -> Vec<Tile> {
    let mut result = Vec::new();
    let north = &(pos.0, pos.1 + 1);
    let south = &(pos.0, pos.1 - 1);
    let east = &(pos.0 + 1, pos.1);
    let west = &(pos.0 - 1, pos.1);

    let mut cur_t: Tile = *t;

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

fn solve(map: &HashMap<(i64, i64), &Tile>, hs: &HashSet<&Tile>) -> (i64, i64) {
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

        println!("bounds = [{:?}, {:?}]", min, max);
        for i in min.0..=max.0 {
            for j in min.1..=max.1 {
                print!(" {}", map.get(&(i, j)).unwrap().num);
            }
            println!("");
        }
        let part1 = map.get(&(min.0, min.1)).unwrap().num
            * map.get(&(min.0, max.1)).unwrap().num
            * map.get(&(max.0, min.1)).unwrap().num
            * map.get(&(max.0, max.1)).unwrap().num;

        let imgdata = image_data(map, &min, &max);
        min.0 = 0;
        min.1 = 0;
        max.0 = 0;
        max.1 = 0;
        for i in &imgdata {
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
        // for i in (min.1..=max.1).rev() {
        //     print!("{:5}: ", i);
        //     for j in min.0..=max.0 {
        //         if imgdata.contains(&(j, i)) {
        //             print!("#");
        //         } else {
        //             print!(".");
        //         }
        //     }
        //     println!("");
        // }

        // let rotat = rotate_hashimg_right(&imgdata, &min, &max);
        // for i in (min.1..=max.1).rev() {
        //     print!("{:5}: ", i);
        //     for j in min.0..=max.0 {
        //         if rotat.contains(&(j, i)) {
        //             print!("#");
        //         } else {
        //             print!(".");
        //         }
        //     }
        //     println!("");
        // }

        let pattern = [
            (0, 0),
            (0, 5),
            (0, 6),
            (0, 11),
            (0, 12),
            (0, 17),
            (0, 18),
            (0, 19),
            (-1, 18),
            (1, 1),
            (1, 4),
            (1, 7),
            (1, 10),
            (1, 13),
            (1, 16),
        ];

        let mut maxserpents = 0;
        let mut cur_img = imgdata.clone();
        for flipped in &[false, true] {
            let mut serpents = 0;
            if *flipped {
                cur_img = flip_hashimg(&cur_img, &min, &max);
            }
            for _ in 0..3 {
                for row in (min.1 + 1)..=max.1 {
                    for col in min.0..=max.0 {
                        let mut count = pattern.len();
                        for p in &pattern {
                            if cur_img.contains(&(col + p.0, row + p.1)) {
                                count -= 1;
                            } else {
                                break;
                            }
                        }
                        if count == 0 {
                            serpents += 1;
                        }
                    }
                }
                if serpents > 0 {
                    break;
                }
                cur_img = rotate_hashimg_right(&cur_img, &min, &max);
            }
            if serpents > maxserpents {
                maxserpents = serpents;
            }
        }
        println!("{} serpents found", maxserpents);

        let part2 = imgdata.len() - maxserpents * pattern.len();
        return (part1, part2 as i64);
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
                        if result.0 != 0 {
                            return result;
                        }
                    }
                }
            }
        }
    }

    (0, 0)
}

fn can_match(tiles: &Vec<Tile>) -> (i64, i64) {
    let hs: HashSet<&Tile> = tiles.iter().collect();

    for tile in &hs {
        let mut h2: HashSet<&Tile> = HashSet::new();
        h2.insert(tile);
        let remainder: HashSet<&Tile> = hs.difference(&h2).map(|x| *x).collect();
        let mut map = HashMap::<(i64, i64), &Tile>::new();
        map.insert((0, 0), tile);
        let answer = solve(&map, &remainder);
        if answer.0 != 0 {
            return answer;
        }
    }

    (0, 0)
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
        let first = m.get(0).unwrap();

        assert_eq!(first.num, 2311);
        assert_eq!(first.img[0], 144);

        assert_eq!(can_match(&m), (20899048083289, 273));
    }

    #[test]
    fn test2() {
        let img: ImgData = [0xff, 0x80, 0x80, 0x80, 0, 0, 0, 0];
        assert_eq!(rotate_img_right(&img), [15, 1, 1, 1, 1, 1, 1, 1]);
    }
}
