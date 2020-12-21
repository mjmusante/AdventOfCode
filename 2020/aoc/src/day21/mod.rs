pub fn run() {}

#[derive(Debug)]
struct Food {
    ingredients: Vec<String>,
    allergens: Vec<String>
}

fn parse(lines: &Vec<String>) -> Vec<Food> {
    let mut result = Vec::new();

    for l in lines {
        let mut food = Food { ingredients: Vec::new(), allergens: Vec::new() };
        let mut ing = true;
        let trimmed : String = l.chars().filter(|c| *c != ',' && *c != '(' && *c != ')').collect();
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
        println!("{:?}", food);
        result.push(food);
    }

    result
}

fn count_safe(foods: &Vec<Food>) -> i64 {
    foods.len() as i64
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
        assert_eq!(count_safe(&foods), 5);
    }
}
