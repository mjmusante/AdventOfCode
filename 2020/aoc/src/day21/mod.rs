use std::collections::{HashMap, HashSet};

use aoc::utils::lines;

pub fn run() {
    let lines = lines("data/21.txt");
    let food = parse(&lines);
    let safe = count_safe(&food);

    println!("Part 1 = {}", safe.0);
    println!("Part 2 = {}", safe.1);
}

#[derive(Debug)]
struct Food {
    ingredients: Vec<String>,
    allergens: Vec<String>,
}

fn parse(lines: &Vec<String>) -> Vec<Food> {
    let mut result = Vec::new();

    for l in lines {
        let mut food = Food {
            ingredients: Vec::new(),
            allergens: Vec::new(),
        };
        let mut ing = true;
        let trimmed: String = l
            .chars()
            .filter(|c| *c != ',' && *c != '(' && *c != ')')
            .collect();
        for s in trimmed.split_whitespace() {
            if ing {
                if s == "contains" {
                    ing = false;
                    continue;
                }
                food.ingredients.push(s.to_string());
            } else {
                food.allergens.push(s.to_string());
            }
        }
        result.push(food);
    }

    result
}

fn count_safe(foods: &Vec<Food>) -> (i64, String) {
    let mut hm = HashMap::<&String, HashSet<&String>>::new();

    for f in foods {
        for a in &f.allergens {
            let mut hs = HashSet::<&String>::new();
            for i in &f.ingredients {
                hs.insert(i);
            }
            if hm.contains_key(a) {
                let existing = hm.get(a).unwrap();
                hs = hs.intersection(existing).map(|x| *x).collect();
            }
            hm.insert(a, hs);
        }
    }

    let mut suspects = HashSet::new();
    for (_, val) in &hm {
        for sus in val {
            suspects.insert(*sus);
        }
    }

    let mut suslist = Vec::<(&String, &String)>::new();
    loop {
        let mut allergen = None;
        for (a, s) in &hm {
            if s.len() == 1 {
                allergen = Some(*a);
                break;
            }
        }

        if let Some(a) = allergen {
            let mut newhm = HashMap::<&String, HashSet<&String>>::new();

            for sus in hm.get(&a).unwrap() {
                suslist.push((a, sus));
                for k in hm.keys() {
                    let mut hs = HashSet::<&String>::new();
                    for i in hm.get(k).unwrap() {
                        if i != sus {
                            hs.insert(i);
                        }
                    }
                    newhm.insert(k, hs);
                }
            }
            hm = newhm;
        } else {
            // we assume there's a solution
            break;
        }
    }
    suslist.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    let output = suslist
        .into_iter()
        .map(|x| x.1.to_string())
        .collect::<Vec<String>>()
        .join(",");

    let mut count = 0;
    for f in foods {
        for i in &f.ingredients {
            if !suspects.contains(i) {
                count += 1;
            }
        }
    }

    (count, output)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let v = vec![
            "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)".to_string(),
            "trh fvjkl sbzzf mxmxvkd (contains dairy)".to_string(),
            "sqjhc fvjkl (contains soy)".to_string(),
            "sqjhc mxmxvkd sbzzf (contains fish)".to_string(),
        ];
        let foods = parse(&v);
        let safe = count_safe(&foods);
        assert_eq!(safe.0, 5);
        assert_eq!(safe.1, "mxmxvkd,sqjhc,fvjkl".to_string());
    }
}
