use lines;

use std::collections::HashMap;

pub fn run() -> (String, String) {
    let mut hm = HashMap::new();
    let lines = lines::lineread("puzzle_data/day12.txt".to_string());
    let istate = lines[0].find(':').unwrap() + 2;

    let mut pots = Pots::new(lines[0].chars().skip(istate).collect::<String>());

    for i in 2..lines.len() {
        if lines[i].chars().skip(9).take(1).collect::<String>() == "#" {
            pots.add_rule(lines[i].chars().take(5).collect::<String>());
        }
    }

    let mut values : Vec<i64> = vec![];
    let mut zeroes : Vec<i64> = vec![];
    let mut count : i64 = 0;

    while !hm.contains_key(&pots.pot) {
        hm.insert(pots.pot.clone(), count);
        count += 1;
        values.push(pots.value());
        zeroes.push(pots.zero);
        let next = pots.apply_rules();
        pots = next;
    }

    let repeat : i64 = *hm.get(&pots.pot).unwrap();
    let newgens = 50_000_000_000i64 - repeat;
    let multiplier = pots.value() - values[repeat as usize];

    let finalval = newgens * multiplier + values[repeat as usize];

    (values[20].to_string(), finalval.to_string())
}


// ==========================================================================

use std::collections::HashSet;

struct Pots {
    pot: String,
    zero: i64,
    rules: HashSet<i64>
}

impl Pots {
    pub fn new(pot: String) -> Pots {
        let mut p = Pots { pot: pot, zero: 0, rules: HashSet::new() };
        p.trim_empty_pots();
        p
    }

    pub fn value(&self) -> i64 {
        let mut start = 0 - self.zero;
        let mut result = 0;

        for c in self.pot.chars() {
            if c == '#' {
                result += start;
            }
            start += 1;
        }

        result
    }

    pub fn add_rule(&mut self, rule: String) {
        self.rules.insert(Pots::str_to_num(rule));
    }

    pub fn apply_rules(&self) -> Pots {
        let mut accum = 0;
        let mut s = String::from("....");
        s.push_str(&self.pot);
        s.push_str("....");
        let process = s.chars().enumerate().map(|(i, c)| {
            accum >>= 1;
            if c == '#' {
                accum |= 0x10;
            }
            if i > 4 {
                if self.has_rule(&accum) {
                    '#'
                } else {
                    '.'
                }
            } else {
                '.'
            }
        }).collect::<String>();

        let mut p = Pots { pot: process, zero: self.zero + 6, rules: self.rules.clone() };
        p.trim_empty_pots();
        p
    }

    fn has_rule(&self, id: &i64) -> bool {
        self.rules.contains(id)
    }

    fn str_to_num(code: String) -> i64 {
        code.chars().enumerate().map(|(i, c)| {
            if c == '#' {
                1 << i
            } else {
                0
            }
        }).sum::<usize>() as i64
    }

    fn trim_empty_pots(&mut self) {
        if let Some(i) = self.pot.find('#') {
            if i > 0 {
                let newpots = self.pot.chars().skip(i).collect();
                self.pot = newpots;
                self.zero -= i as i64;
            }
        }

        if let Some(i) = self.pot.rfind('#') {
            let newpots = self.pot.chars().take(i + 1).collect();
            self.pot = newpots;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vec_of_strings {
        ($($x:expr),*) => (vec![$($x.to_string()),*]);
    }

    #[test]
    fn day12_test1() {
        let p = Pots::new("#..#.#..##......###...###".to_string());
        assert_eq!(p.value(), 0 + 3 + 5 + 8 + 9 + 16 + 17 + 18 + 22 + 23 + 24);
    }

    #[test]
    fn day12_test2() {
        let mut p = Pots::new("#....##....#####...#######....#.#..##".to_string());
        p.zero += 2; // adjust for AoC-given example
        assert_eq!(p.value(), 325);
    }

    #[test]
    fn day12_test3() {
        assert_eq!(Pots::str_to_num(".....".to_string()), 0);
        assert_eq!(Pots::str_to_num("#....".to_string()), 1);
        assert_eq!(Pots::str_to_num(".#...".to_string()), 2);
        assert_eq!(Pots::str_to_num("##...".to_string()), 3);
        assert_eq!(Pots::str_to_num("..#..".to_string()), 4);
        assert_eq!(Pots::str_to_num("#.#..".to_string()), 5);
        assert_eq!(Pots::str_to_num(".##..".to_string()), 6);
        assert_eq!(Pots::str_to_num("###..".to_string()), 7);
        assert_eq!(Pots::str_to_num("...#.".to_string()), 8);
        assert_eq!(Pots::str_to_num("#..#.".to_string()), 9);
        assert_eq!(Pots::str_to_num(".#.#.".to_string()), 10);
        assert_eq!(Pots::str_to_num("##.#.".to_string()), 11);
        assert_eq!(Pots::str_to_num("..##.".to_string()), 12);
        assert_eq!(Pots::str_to_num("#.##.".to_string()), 13);
        assert_eq!(Pots::str_to_num(".###.".to_string()), 14);
        assert_eq!(Pots::str_to_num("####.".to_string()), 15);
        assert_eq!(Pots::str_to_num("....#".to_string()), 16);
        assert_eq!(Pots::str_to_num("#...#".to_string()), 17);
        assert_eq!(Pots::str_to_num(".#..#".to_string()), 18);
        assert_eq!(Pots::str_to_num("##..#".to_string()), 19);
        assert_eq!(Pots::str_to_num("..#.#".to_string()), 20);
        assert_eq!(Pots::str_to_num("#.#.#".to_string()), 21);
        assert_eq!(Pots::str_to_num(".##.#".to_string()), 22);
        assert_eq!(Pots::str_to_num("###.#".to_string()), 23);
        assert_eq!(Pots::str_to_num("...##".to_string()), 24);
        assert_eq!(Pots::str_to_num("#..##".to_string()), 25);
        assert_eq!(Pots::str_to_num(".#.##".to_string()), 26);
        assert_eq!(Pots::str_to_num("##.##".to_string()), 27);
        assert_eq!(Pots::str_to_num("..###".to_string()), 28);
        assert_eq!(Pots::str_to_num("#.###".to_string()), 29);
        assert_eq!(Pots::str_to_num(".####".to_string()), 30);
        assert_eq!(Pots::str_to_num("#####".to_string()), 31);
    }

    #[test]
    fn day12_test4() {
        let mut p = Pots::new("..#..".to_string());
        p.add_rule("..#..".to_string());
        assert!(p.has_rule(&4));
    }

    #[test]
    fn day12_test5() {
        let mut p = Pots::new("..#..".to_string());
        p.add_rule("..#..".to_string());
        let newp = p.apply_rules();
        assert_eq!(newp.value(), 2);
    }

    #[test]
    fn day12_test6() {
        let v = vec_of_strings![
            "...##",
            "..#..",
            ".#...",
            ".#.#.",
            ".#.##",
            ".##..",
            ".####",
            "#.#.#",
            "#.###",
            "##.#.",
            "##.##",
            "###..",
            "###.#",
            "####."
        ];

        let mut p = Pots::new("#..#.#..##......###...###".to_string());
        for rule in v {
            p.add_rule(rule.to_string());
        }
        for _ in 0..20 {
            let next = p.apply_rules();
            p = next;
        }
        assert_eq!(p.value(), 325);
    }
}
