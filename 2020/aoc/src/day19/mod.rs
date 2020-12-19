pub fn run() {}

#[derive(Debug)]
enum Match {
    Str(char),
    Index(i64)
}

#[derive(Debug)]
struct Rule {
    num: i64,
    alt: Vec<Vec<Match>>,
}

#[derive(Debug)]
struct Messages {
    rules: Vec<Rule>,
    messages: Vec<String>
}

impl Messages {
    pub fn count_match(&self) -> i64 {
        self.messages.len() as i64
    }
}

fn parse(lines: &Vec<String>) -> Messages {
    let mut ans = Messages { rules: Vec::new(), messages: Vec::new() };
    let mut rules = true;

    for l in lines {
        if l == "" {
            rules = false;
            continue;
        }

        if rules {
            let mut p = l.split(":");
            let num : i64 = p.next().unwrap().parse().unwrap();
            let mut rule = Rule { num, alt: Vec::new() };

            let mut nr = Vec::new();

            for r in p.next().unwrap().split_whitespace() {
                if r == "|" {
                    rule.alt.push(nr);
                    nr = Vec::new();
                } else {
                    if r.starts_with("\"") {
                        let mut x = r.chars();
                        x.next();
                        nr.push(Match::Str(x.next().unwrap()));
                    } else {
                        nr.push(Match::Index(r.parse().unwrap()));
                    }
                }
            }
            rule.alt.push(nr);
            ans.rules.push(rule);
        } else {
            ans.messages.push(l.to_string());
        }
    }

    ans.rules.sort_by(|a, b| a.num.cmp(&b.num));

    println!("{:?}", ans);

    ans
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_data() -> Vec<String> {
        vec![
            "1: 2 3 | 3 2".to_string(),
            "5: \"b\"".to_string(),
            "2: 4 4 | 5 5".to_string(),
            "3: 4 5 | 5 4".to_string(),
            "0: 4 1 5".to_string(),
            "4: \"a\"".to_string(),
            "".to_string(),
            "ababbb".to_string(),
            "bababa".to_string(),
            "abbbab".to_string(),
            "aaabbb".to_string(),
            "aaaabbb".to_string(),
        ]
    }

    #[test]
    fn test1() {
        let v = test_data();
        let m = parse(&v);

        assert_eq!(m.count_match(), 0);
    }
}
