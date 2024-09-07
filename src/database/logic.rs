use std::collections::{HashMap, HashSet};

pub fn filter_recipes_with_ingredient(filter: &HashSet<char>, all_ingredients: &HashSet<char>, recipes: &Vec<Vec<char>>) {
    for recipe in recipes {
        for keyword in filter {
            if recipe.contains(keyword) {
                if no_other_keywords(filter, recipe, all_ingredients) {
                    break;
                }
            }
        }
    }
}

pub fn no_other_keywords(filter: &HashSet<char>, list: &Vec<char>, all_ingredients: &HashSet<char>) -> bool{
    let difference: HashSet<_> = all_ingredients.difference(&filter).cloned().collect();
    for keyword in difference {
        if list.contains(&keyword) {
            return false;
        }
    }
    list.iter().for_each(|c| print!("{}, ", c));
    println!("\n");

    return true;
}