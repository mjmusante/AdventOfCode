use aoc::utils::lines;

#[derive(Debug)]
struct Entry {
    byr: bool,
    iyr: bool,
    eyr: bool,
    hgt: bool,
    hcl: bool,
    ecl: bool,
    pid: bool,
    cid: bool,
}

impl Entry {
    pub fn new() -> Entry {
        Entry {
            byr: false,
            iyr: false,
            eyr: false,
            hgt: false,
            hcl: false,
            ecl: false,
            pid: false,
            cid: false,
        }
    }

    pub fn valid(&self) -> bool {
        self.byr && self.iyr && self.eyr && self.hgt && self.hcl && self.ecl && self.pid
    }
    pub fn add_line1(&mut self, line: &String) {
        for field in line.split(" ").map(|entry| entry.split(":").nth(0)) {
            match field.unwrap() {
                "byr" => self.byr = true,
                "iyr" => self.iyr = true,
                "eyr" => self.eyr = true,
                "hgt" => self.hgt = true,
                "hcl" => self.hcl = true,
                "ecl" => self.ecl = true,
                "pid" => self.pid = true,
                "cid" => self.cid = true,
                _ => {
                    panic!("Invalid field {}", field.unwrap());
                }
            };
        }
    }

    pub fn add_line2(&mut self, line: &String) {
        for field in line.split(" ") {
            let mut p = field.split(":");
            let ftype = p.next().unwrap();
            let fval = p.next().unwrap();

            match ftype {
                "byr" => self.byr = Entry::valid_year(fval, 1920, 2002),
                "iyr" => self.iyr = Entry::valid_year(fval, 2010, 2020),
                "eyr" => self.eyr = Entry::valid_year(fval, 2020, 2030),
                "hgt" => self.hgt = Entry::valid_height(fval),
                "hcl" => self.hcl = Entry::valid_hair(fval),
                "ecl" => self.ecl = Entry::valid_eye(fval),
                "pid" => self.pid = Entry::valid_id(fval),
                "cid" => self.cid = true,
                _ => {
                    panic!("Invalid field {}", ftype);
                }
            };
        }
    }

    fn valid_year(txt: &str, min: i64, max: i64) -> bool {
        let year: i64 = txt.parse().unwrap();
        year >= min && year <= max
    }

    fn valid_height(txt: &str) -> bool {
        let l = txt.len();
        let min;
        let max;

        if &txt[l - 2..l] == "in" {
            min = 59;
            max = 76;
        } else if &txt[l - 2..l] == "cm" {
            min = 150;
            max = 193;
        } else {
            return false;
        }

        let height: i64 = txt
            .chars()
            .filter(|c| c.is_digit(10))
            .collect::<String>()
            .parse()
            .unwrap();

        height >= min && height <= max
    }

    fn valid_hair(txt: &str) -> bool {
        if &txt[0..1 as usize] != "#" {
            return false;
        }

        txt[1 as usize..].chars().filter(|c| c.is_digit(16)).count() == 6
    }

    fn valid_eye(txt: &str) -> bool {
        match txt {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
            _ => false,
        }
    }

    fn valid_id(txt: &str) -> bool {
        if txt.len() != 9 {
            return false;
        }

        txt.chars().filter(|c| c.is_digit(10)).count() == 9
    }
}

pub fn run() {
    let xlines = lines("data/04.txt");

    let mut e = Entry::new();
    let mut part1 = 0;
    let mut part2 = 0;

    for l in &xlines {
        if l == "" {
            if e.valid() {
                part1 += 1;
            }
            e = Entry::new();
        } else {
            e.add_line1(&l);
        }
    }
    if e.valid() {
        part1 += 1;
    }
    println!("Part 1 = {}", part1);

    for l in xlines {
        if l == "" {
            if e.valid() {
                part2 += 1;
            }
            e = Entry::new();
        } else {
            e.add_line2(&l);
        }
    }
    if e.valid() {
        part2 += 1;
    }
    println!("Part 2 = {}", part2);
}
