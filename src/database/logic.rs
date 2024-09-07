use std::collections::{HashMap, HashSet};

pub fn filter_recipes_with_ingredient(filter: &HashSet<u32>, all_ingredients: &HashSet<u32>, recipes: &HashMap<u32, Vec<u32>>) -> Vec<u32> {
    let mut filtered: Vec<u32> = Vec::new();
    for (recipe, ingredients) in recipes {
        for keyword in filter {
            if ingredients.contains(keyword) {
                if no_other_keywords(filter, all_ingredients, ingredients) {
                    filtered.push(*recipe);
                    break;
                }
            }
        }
    }

    filtered
}

pub fn no_other_keywords(filter: &HashSet<u32>, all_ingredients: &HashSet<u32>, ingredients: &Vec<u32>) -> bool{
    let difference: HashSet<_> = all_ingredients.difference(&filter).cloned().collect();
    for keyword in difference {
        if ingredients.contains(&keyword) {
            return false;
        }
    }

    return true;
}