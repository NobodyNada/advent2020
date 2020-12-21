use std::io::{self, BufRead};
use std::collections::{HashMap, HashSet};
use regex::Regex;
use lazy_static::lazy_static;

struct Recipe {
    ingredients: HashSet<String>,
    allergens: HashSet<String>
}

impl Recipe {
    fn parse(input: &str) -> Option<Recipe> {
        lazy_static! {
            static ref REGEX: Regex = Regex::new(r"^((?:\w++ ?)+) \(contains ([\w\s,]+)\)$").unwrap();
        };

        let matches = REGEX.captures(input)?;

        let ingredients = matches.get(1)?.as_str().split_whitespace();
        let allergens = matches.get(2)?.as_str().split(", ");

        Some(Recipe {
            ingredients: ingredients.map(str::to_string).collect(),
            allergens: allergens.map(str::to_string).collect()
        })
    }
}

#[derive(Default)]
struct Ingredient {
    possible_allergens: HashSet<String>
}

pub fn run() {
    let recipes = io::stdin().lock().lines()
        .map(|line| line.expect("read error"))
        .map(|line| Recipe::parse(&line).expect("invalid input"))
        .collect::<Vec<Recipe>>();

    let mut ingredients = HashMap::<String, Ingredient>::new();
    for recipe in recipes.iter() {
        // Add all of this recipe's allergens to each of this recipe's ingredients
        for ingredient in recipe.ingredients.iter() {
            let entry = if let Some(entry) = ingredients.get_mut(ingredient) { entry }
                        else { &mut *ingredients.entry(ingredient.to_string()).or_default() };
            entry.possible_allergens.extend(recipe.allergens.iter().cloned());
        }
    }

    for (ingredient_name, ingredient) in ingredients.iter_mut() {
        // If a recipe contains one of our possible allergens but doesn't contain us,
        // then remove that allergen from our list of possibilities.
        recipes.iter()
            .filter(|recipe| !recipe.ingredients.contains(ingredient_name))
            .flat_map(|recipe| recipe.allergens.iter())
            .for_each(|impossible_allergen| { ingredient.possible_allergens.remove(impossible_allergen); });
    }

    // Find occurences of ingredients with no possible allergens
    let result = recipes.iter()
        .flat_map(|recipe| recipe.ingredients.iter())
        .map(|name| ingredients.get(name).expect("missing ingredient"))
        .filter(|ingredient| ingredient.possible_allergens.is_empty())
        .count();
    println!("{}", result);
}
