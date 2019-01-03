use lines;

pub fn run() -> (String, String) {
    let lines = lines::lineread(String::from("puzzle_data/day15.txt"));

    let (mut g, xs, ys) = make_grid(&lines);
    let num_elves = elf_count(&g);
    let mut rounds = 0;
    while find_path(&mut g, xs, ys) {
        rounds += 1;
    }
    let part1 = format!("{}", rounds * sum_hp(&g));

    let mut remaining = elf_count(&g);
    let mut power = 3;
    while remaining < num_elves {
        power *= 2;
        let (mut g, xs, ys) = make_grid(&lines);
        rounds = 0;
        while find_path_by_power(&mut g, xs, ys, power) {
            rounds += 1;
        }
        remaining = elf_count(&g);
    }

    let mut low_power = power / 2;
    let mut high_power = power;
    loop {
        if high_power - low_power < 2 {
            break;
        }
        power = (low_power + high_power) / 2;
        let (newg, _, _) = make_grid(&lines);
        g = newg;
        rounds = 0;
        while find_path_by_power(&mut g, xs, ys, power) {
            rounds += 1;
        }
        remaining = elf_count(&g);
        if remaining == num_elves {
            high_power = power;
        } else {
            low_power = power;
        }
    }
    let part2 = format!("{}", rounds * sum_hp(&g));

    (part1, part2)
}

use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
enum CreatureType {
    Elf,
    Goblin,
}

#[derive(Debug, Copy, Clone)]
enum Item {
    Creature(CreatureType, i32),
    Open,
    Wall,
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (&Item::Wall, &Item::Wall) | (&Item::Open, &Item::Open) => true,
            (&Item::Creature(a, _), &Item::Creature(b, _)) => a == b,
            (_, _) => false,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Tile {
    killed: bool,
    item: Item,
    pos: Point,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Copy, Clone)]
enum Action {
    Attack(Point),
    MoveTo(Point),
    DoNothing,
    Abort,
}

impl Tile {
    pub fn make(c: char, row: usize, col: usize) -> Tile {
        let item = match c {
            '.' => Item::Open,
            '#' => Item::Wall,
            'E' => Item::Creature(CreatureType::Elf, 200),
            'G' => Item::Creature(CreatureType::Goblin, 200),
            _ => panic!("bad char"),
        };

        Tile {
            killed: false,
            item: item,
            pos: Point { x: col, y: row },
        }
    }

    pub fn make_point(c: char, p: &Point) -> Tile {
        Tile::make(c, p.y, p.x)
    }

    pub fn adjust(&mut self, row: usize, col: usize) {
        self.pos.x = col;
        self.pos.y = row;
    }

    // returns true if creature taking the hit is still alive
    pub fn take_hit(&mut self, power: usize) -> bool {
        let new_item = match self.item {
            Item::Creature(CreatureType::Goblin, hp) => {
                Item::Creature(CreatureType::Goblin, hp - power as i32)
            }
            Item::Creature(CreatureType::Elf, hp) => Item::Creature(CreatureType::Elf, hp - 3),
            _ => {
                panic!("nooooo! {:?}", self.item);
            }
        };
        self.item = new_item;

        match new_item {
            Item::Creature(_, hp) => hp > 0,
            _ => {
                panic!("unreachable");
            }
        }
    }

    pub fn creature_type(&self) -> Option<CreatureType> {
        match self.item {
            Item::Creature(c, _) => Some(c),
            _ => None,
        }
    }
}

fn is_creature(a: &Item) -> bool {
    match a {
        Item::Creature(_, _) => true,
        _ => false,
    }
}

fn elf_count(grid: &HashMap<Point, Tile>) -> usize {
    let mut result = 0;

    for (_, g) in grid.into_iter() {
        result += match g.item {
            Item::Creature(CreatureType::Elf, _) => 1,
            _ => 0,
        }
    }

    result
}

fn surround(pos: &Point) -> Vec<Point> {
    let mut locs = vec![];

    locs.push(Point {
        x: pos.x,
        y: pos.y - 1,
    }); // north
    locs.push(Point {
        x: pos.x - 1,
        y: pos.y,
    }); // west
    locs.push(Point {
        x: pos.x + 1,
        y: pos.y,
    }); // east
    locs.push(Point {
        x: pos.x,
        y: pos.y + 1,
    }); // south

    locs
}

fn can_attack(start: &Tile, grid: &HashMap<Point, Tile>) -> Option<Point> {
    let tiles = surround(&start.pos);
    let mut best = 201;
    let mut result = None;
    let attacker = match start.item {
        Item::Creature(a, _) => a,
        _ => {
            panic!("Attacking from invalid location");
        }
    };

    for t in tiles {
        match grid.get(&t).unwrap().item {
            Item::Creature(enemy, hp) => {
                if attacker != enemy {
                    if hp < best {
                        result = Some(t);
                        best = hp;
                    }
                }
            }
            _ => {}
        }
    }

    result
}

fn get_best(start: &Tile, dests: &Vec<Point>, grid: &HashMap<Point, Tile>) -> Action {
    let mut hm = HashMap::new();
    let mut dist = 1;
    let mut search: Vec<Point> = vec![];

    if dests.len() == 0 {
        return Action::DoNothing;
    }

    if let Some(a) = can_attack(start, grid) {
        // println!("Starting from {:?} we're attempting to attack {:?}", start.pos, a);
        return Action::Attack(a);
    }

    search.append(&mut surround(&start.pos));

    while search.len() > 0 {
        let mut next_search = vec![];

        for i in search {
            let g = grid.get(&i).unwrap();
            if g.item == Item::Open && !hm.contains_key(&i) {
                hm.insert(i.clone(), dist);
                for l in surround(&i) {
                    if !hm.contains_key(&l) {
                        next_search.push(l);
                    }
                }
            }
        }

        dist += 1;
        search = next_search;
    }

    // println!("Starting from {:?} we found {} potential locations", start.pos, hm.len());

    // now check the destination locations to see which one is closest
    let mut best = dist;
    let mut first = Point { x: best, y: best };
    for d in dests {
        assert!(d.x != start.pos.x || d.y != start.pos.y);
        for s in surround(d) {
            if s == start.pos {
                println!("Found adjacent enemy start={:?}, d={:?}", start, d);
                return Action::Abort;
            } else if hm.contains_key(&s) {
                let n = hm.get(&s).unwrap();
                if n < &best {
                    // println!("New best: {} @ {:?}", n, s);
                    best = *n;
                    first.x = s.x;
                    first.y = s.y;
                } else if *n == best {
                    // println!("Same best @ {:?}", s);
                    if s.y < first.y || (s.y == first.y && s.x < first.x) {
                        // println!("    (overrides previous)");
                        first.x = s.x;
                        first.y = s.y;
                    }
                }
            }
        }
    }

    if best == dist {
        // println!("Starting from {:?}, there is nowhere to go", start.pos);
        // no attack vectors available
        return Action::DoNothing;
    }

    let mut target_val = hm.get(&first).unwrap() - 1;
    let mut step = first;
    while target_val > 0 {
        for s in surround(&step) {
            if hm.contains_key(&s) {
                let next = *hm.get(&s).unwrap();
                if next == target_val {
                    target_val -= 1;
                    step = s;
                    break;
                }
            }
        }
    }

    // println!("Starting from {:?} we're attempting to move to {:?}", start.pos, step);
    Action::MoveTo(step)
}

fn reading_order(a: &Point, b: &Point) -> Ordering {
    if a.y == b.y {
        a.x.cmp(&b.x)
    } else {
        a.y.cmp(&b.y)
    }
}

fn make_grid(text_grid: &Vec<String>) -> (HashMap<Point, Tile>, usize, usize) {
    let xsize = text_grid[0].len();
    let ysize = text_grid.len();

    let mut lnum = 0;
    let mut grid = HashMap::new();

    for g in text_grid
        .iter()
        .map(|l| {
            lnum += 1;
            l.chars()
                .enumerate()
                .map(|(i, c)| Tile::make(c, lnum - 1, i))
                .collect::<Vec<Tile>>()
        }).flat_map(|x| x)
        .collect::<Vec<Tile>>()
    {
        grid.insert(g.pos, g);
    }

    (grid, xsize, ysize)
}

fn find_path(grid: &mut HashMap<Point, Tile>, xsize: usize, ysize: usize) -> bool {
    find_path_by_power(grid, xsize, ysize, 3)
}

fn find_path_by_power(
    grid: &mut HashMap<Point, Tile>,
    xsize: usize,
    ysize: usize,
    power: usize,
) -> bool {
    let mut v = vec![];
    let mut nums = HashMap::new();

    for (_, g) in grid.into_iter() {
        match g.item {
            Item::Creature(c, _) => {
                *nums.entry(c).or_insert(0) += 1;
                v.push(g.clone());
            }
            _ => (),
        }
    }

    if nums.get(&CreatureType::Elf) == None || nums.get(&CreatureType::Goblin) == None {
        return false;
    }

    v.sort_by(|a, b| reading_order(&a.pos, &b.pos));
    let mut living = vec![true; v.len()];

    let mut targets = v.clone();

    for i in 0..v.len() {
        let src = &v[i];
        if !living[i] {
            continue;
        }
        if (*nums.get(&CreatureType::Elf).unwrap() == 0)
            || (*nums.get(&CreatureType::Goblin).unwrap() == 0)
        {
            return false;
        }
        let mut dests: Vec<Point> = targets
            .clone()
            .into_iter()
            .filter(|d| (d.item != src.item) && is_creature(&grid.get(&d.pos).unwrap().item))
            .map(|x| x.pos)
            .collect();

        match get_best(src, &dests, &grid) {
            Action::DoNothing => {}
            Action::MoveTo(b) => {
                let mut m = grid.remove(&src.pos).unwrap();
                grid.insert(src.pos, Tile::make('.', src.pos.y, src.pos.x));

                m.adjust(b.y, b.x);
                grid.remove(&b);
                grid.insert(b, m);
                targets = targets
                    .iter()
                    .filter(|d| d.pos != src.pos)
                    .map(|d| *d)
                    .collect();
                targets.push(m.clone());

                if let Some(a) = can_attack(&m, &grid) {
                    // println!("After moving, we can attack {:?}", a);
                    let mut m = grid.remove(&a).unwrap();
                    if m.take_hit(power) {
                        grid.insert(a, m);
                    } else {
                        for j in 0..v.len() {
                            if v[j].pos == a {
                                living[j] = false;
                            }
                        }
                        if let Some(c) = m.creature_type() {
                            *nums.entry(c).or_insert(0) -= 1;
                        }
                        grid.insert(a, Tile::make_point('.', &a));
                        targets = targets.iter().filter(|d| d.pos != a).map(|d| *d).collect();
                    }
                }
            }
            Action::Attack(b) => {
                let mut m = grid.remove(&b).unwrap();
                if m.take_hit(power) {
                    grid.insert(b, m);
                } else {
                    for j in 0..v.len() {
                        if v[j].pos == b {
                            living[j] = false;
                        }
                    }
                    if let Some(c) = m.creature_type() {
                        *nums.entry(c).or_insert(0) -= 1;
                    }
                    grid.insert(b, Tile::make_point('.', &b));
                    targets = targets.iter().filter(|d| d.pos != b).map(|d| *d).collect();
                }
            }
            Action::Abort => {
                show_grid(grid, xsize, ysize);
                panic!("Abort");
            }
        }
    }

    true
}

fn sum_hp(grid: &HashMap<Point, Tile>) -> i32 {
    let mut total_hp = 0;
    for (_, v) in grid {
        match v.item {
            Item::Creature(_, hp) => total_hp += hp,
            _ => {}
        };
    }

    total_hp
}

fn show_grid(grid: &HashMap<Point, Tile>, xsize: usize, ysize: usize) -> i32 {
    let mut total_hp = 0;

    for row in 0..ysize {
        let mut s = String::from("");
        for col in 0..xsize {
            match grid.get(&(Point { x: col, y: row })).unwrap().item {
                Item::Creature(CreatureType::Goblin, hp) => {
                    print!("G");
                    s.push_str(&format!("G({}) ", hp));
                    total_hp += hp;
                }
                Item::Creature(CreatureType::Elf, hp) => {
                    print!("E");
                    s.push_str(&format!("E({}) ", hp));
                    total_hp += hp;
                }
                Item::Wall => {
                    print!("#");
                }
                Item::Open => {
                    print!(".");
                }
            };
        }
        println!("   {}", s);
    }

    total_hp
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day15_test1() {
        let v: Vec<String> = [
            String::from("#########"),
            String::from("#G..G..G#"),
            String::from("#.......#"),
            String::from("#.......#"),
            String::from("#G..E..G#"),
            String::from("#.......#"),
            String::from("#.......#"),
            String::from("#G..G..G#"),
            String::from("#########"),
        ]
            .to_vec();

        let (mut g, xs, ys) = make_grid(&v);
        show_grid(&g, xs, ys);
        find_path(&mut g, xs, ys);
        show_grid(&g, xs, ys);
        find_path(&mut g, xs, ys);
        show_grid(&g, xs, ys);
        find_path(&mut g, xs, ys);
        show_grid(&g, xs, ys);

        let mut rounds = 3;
        while find_path(&mut g, xs, ys) {
            rounds += 1;
        }
        assert_eq!(rounds, 18);
        assert_eq!(sum_hp(&g), 1546);
    }

    #[test]
    fn day15_test2() {
        let v: Vec<String> = [
            String::from("#######"),
            String::from("#.G...#"),
            String::from("#...EG#"),
            String::from("#.#.#G#"),
            String::from("#..G#E#"),
            String::from("#.....#"),
            String::from("#######"),
        ]
            .to_vec();

        let mut round = 0;

        let (mut g, xs, ys) = make_grid(&v);
        println!("Round 0");
        show_grid(&g, xs, ys);

        find_path(&mut g, xs, ys);
        round += 1;
        println!("Round {}", round);
        show_grid(&g, xs, ys);

        find_path(&mut g, xs, ys);
        round += 1;
        println!("Round {}", round);
        show_grid(&g, xs, ys);

        for _ in 2..23 {
            find_path(&mut g, xs, ys);
            round += 1;
        }
        println!("Round {}", round);
        show_grid(&g, xs, ys);

        for _ in 24..29 {
            find_path(&mut g, xs, ys);
            round += 1;
            println!("Round {}", round);
            show_grid(&g, xs, ys);
        }

        for _ in 29..47 {
            find_path(&mut g, xs, ys);
            round += 1;
        }
        println!("Round {}", round);
        show_grid(&g, xs, ys);

        assert!(find_path(&mut g, xs, ys));
        round += 1;
        println!("Round {}", round);
        show_grid(&g, xs, ys);
        assert!(!find_path(&mut g, xs, ys));

        assert_eq!(round, 47);
        assert_eq!(show_grid(&g, xs, ys), 590);
        assert_eq!(sum_hp(&g), 590);
    }

    fn run_test(v: Vec<String>) -> (i32, i32) {
        let (mut g, xs, ys) = make_grid(&v);

        let mut rounds = 0;
        show_grid(&g, xs, ys);
        while find_path(&mut g, xs, ys) {
            rounds += 1;
            show_grid(&g, xs, ys);
        }

        show_grid(&g, xs, ys);
        (rounds, sum_hp(&g))
    }

    #[test]
    fn day15_test3() {
        let v: Vec<String> = [
            String::from("#######"),
            String::from("#G..#E#"),
            String::from("#E#E.E#"),
            String::from("#G.##.#"),
            String::from("#...#E#"),
            String::from("#...E.#"),
            String::from("#######"),
        ]
            .to_vec();

        assert_eq!(run_test(v), (37, 982));
    }

    #[test]
    fn day15_test4() {
        let v: Vec<String> = [
            String::from("#######"),
            String::from("#E..EG#"),
            String::from("#.#G.E#"),
            String::from("#E.##E#"),
            String::from("#G..#.#"),
            String::from("#..E#.#"),
            String::from("#######"),
        ]
            .to_vec();

        assert_eq!(run_test(v), (46, 859));
    }

    #[test]
    fn day15_test5() {
        let v: Vec<String> = [
            String::from("#######"),
            String::from("#E.G#.#"),
            String::from("#.#G..#"),
            String::from("#G.#.G#"),
            String::from("#G..#.#"),
            String::from("#...E.#"),
            String::from("#######"),
        ]
            .to_vec();

        // let (mut g, xs, ys) = make_grid(&v);
        // show_grid(&g, xs, ys);
        // find_path(&mut g, xs, ys);
        // show_grid(&g, xs, ys);
        // find_path(&mut g, xs, ys);
        // show_grid(&g, xs, ys);
        // assert!(false);

        assert_eq!(run_test(v), (35, 793));
    }

    #[test]
    fn day15_test6() {
        let v: Vec<String> = [
            String::from("#######"),
            String::from("#.E...#"),
            String::from("#.#..G#"),
            String::from("#.###.#"),
            String::from("#E#G#G#"),
            String::from("#...#G#"),
            String::from("#######"),
        ]
            .to_vec();

        assert_eq!(run_test(v), (54, 536));
    }

    #[test]
    fn day15_test7() {
        let v: Vec<String> = [
            String::from("#########"),
            String::from("#G......#"),
            String::from("#.E.#...#"),
            String::from("#..##..G#"),
            String::from("#...##..#"),
            String::from("#...#...#"),
            String::from("#.G...G.#"),
            String::from("#.....G.#"),
            String::from("#########"),
        ]
            .to_vec();

        assert_eq!(run_test(v), (20, 937));
    }

    #[test]
    fn day15_test8() {
        let v1: Vec<String> = [
            String::from("#######"),
            String::from("#.E..G#"),
            String::from("#.#####"),
            String::from("#G#####"),
            String::from("#######"),
        ]
            .to_vec();
        assert_eq!(run_test(v1), (34, 301));

        let v2 = [
            String::from("####"),
            String::from("##E#"),
            String::from("#GG#"),
            String::from("####"),
        ]
            .to_vec();
        assert_eq!(run_test(v2), (67, 200));
    }
}
