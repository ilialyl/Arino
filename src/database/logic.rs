use std::collections::{HashMap, HashSet};

pub fn filter_dishes_with_input_ingredients(input_ingredients: &HashSet<u32>, all_ingredients: &HashSet<u32>, dish_recipes: &HashMap<u32, Vec<u32>>) -> Vec<u32> {
    let mut filtered: Vec<u32> = Vec::new();
    for (dish_name, ingredients_vec) in dish_recipes {
        for keyword in input_ingredients {
            if ingredients_vec.contains(keyword) {
                if filter_dishes_with_other_ingredients(input_ingredients, all_ingredients, ingredients_vec) {
                    filtered.push(*dish_name);
                    break;
                }
            }
        }
    }

    filtered
}

pub fn filter_dishes_with_other_ingredients(input_ingredients: &HashSet<u32>, all_ingredients: &HashSet<u32>, ingredients_vec: &Vec<u32>) -> bool {
    let other_ingredients: HashSet<_> = all_ingredients.difference(&input_ingredients).cloned().collect();

    for ingredient in other_ingredients {
        if ingredients_vec.contains(&ingredient) {
            return false;
        }
    }

    return true;
}