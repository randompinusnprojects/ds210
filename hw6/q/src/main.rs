use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::error::Error;
use std::io::{stdin,stdout,Write};

fn main() -> Result<(), Box<dyn Error>> {
    
    let mut path = "./categories_ingredients.txt";
    let categories_ingredients = read_to_hashmap(path)?;
    path = "./people_categories.txt";
    let people_categories = read_to_hashmap(path)?;
    path = "./recipes.txt";
    let recipes = read_to_hashmap(path)?;

    let mut given_recipe = String::new();
    print!("Please enter 'Recipe###' where ### varies from 0-999: ");
    stdout().flush()?;
    stdin().read_line(&mut given_recipe)?;
    given_recipe = given_recipe.trim().to_lowercase();
    
    // for random testing
    //let mut given_number = String::new();
    //print!("Please enter any number between 0-999: ");
    //stdout().flush()?;
    //stdin().read_line(&mut given_number)?;
    //let given_number = given_number.trim().parse::<usize>().unwrap();
    //let keys: Vec<&String> = people_categories.keys().collect();
    //let given_person = keys[given_number];

    // or
    let mut given_person = String::new();
    print!("Please enter person's full name or 'popular recipes': ");
    stdout().flush()?;
    stdin().read_line(&mut given_person)?;
    let given_person = &given_person.trim().to_lowercase();
    
    if given_person == "popular recipes" {
        let mut freq_collector = HashMap::new();
        for person in people_categories.keys() {
            let liked_ingredients = person_to_liked_ingredients(person, &people_categories, &categories_ingredients);
            
            for recipe in recipes.keys() {
                let ingredients = recipe_to_ingredients(recipe, &recipes);
                if like_checker(&ingredients, &liked_ingredients) {
                    *freq_collector.entry(recipe).or_insert(0) += 1;
                }
            }
        }

        let mut max_finder: Vec<(&String, &usize)> = freq_collector.iter().map(|(k, v)| (*k, v)).collect();
        max_finder.sort_by(|a, b| {
            b.1.cmp(a.1)
               .then_with(|| b.0.cmp(a.0))
            });
        
        for (recipe, count) in max_finder.iter().take(3) {
            println!("{}: {}", recipe, count);
        }
        
        // println!("{:?}", max_finder)
    }
    
    else if given_recipe.is_empty() {
        println!("\n{} likes the following recipes:", given_person);
        let liked_ingredients = person_to_liked_ingredients(given_person, &people_categories, &categories_ingredients);
        for recipe in recipes.keys() {
            let ingredients = recipe_to_ingredients(recipe, &recipes);
            if like_checker(&ingredients, &liked_ingredients) {
                println!("- {}", recipe);
            }
        }
    }

    else { 
        let liked_ingredients = person_to_liked_ingredients(given_person, &people_categories, &categories_ingredients);
        let ingredients = recipe_to_ingredients(&given_recipe, &recipes);
        if like_checker(&ingredients, &liked_ingredients)  {
            println!("{} likes {}", given_person, given_recipe);
        } else {
            println!("{} doesn't like {}", given_person, given_recipe);
        }
    }
    


    Ok(())
}

fn read_to_hashmap(path: &str) -> Result<HashMap<String, Vec<String>>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut map = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        if let Some((category, items)) = line.split_once(':') {
            let key = category.trim().to_lowercase();
            let mut values: Vec<String> = items
                .split(',')
                .map(|s| s.trim().to_string())
                .collect();

                if values.last().map_or(false, |s| s.is_empty()) {
                    values.pop();
                }
            map.insert(key, values);
        }
    }


    Ok(map)
}

fn recipe_to_ingredients(recipe: &String, recipes: &HashMap<String, Vec<String>>) -> Vec<String> {
    match recipes.get(recipe) {
        Some(ings) => return ings.to_vec(),
        None => {
            panic!("'{}' is invalid", recipe);
        }
    };
}

fn person_to_liked_ingredients(target: &str, persons: &HashMap<String, Vec<String>>, categories: &HashMap<String, Vec<String>>) -> HashSet<String> {
    let liked_categories = match persons.get(target) {
        Some(ings) => ings,
        None => {
            panic!("'{}' is invalid", target);
        }
    };

    let mut liked_ingredients = HashSet::new();
    for category in liked_categories {
        if let Some(ings) = categories.get(category) {
            for ing in ings {
                liked_ingredients.insert(ing.clone());
            }
        }
    }

    return liked_ingredients;
}

fn like_checker(
    ingredients: &Vec<String>,
    liked_ingredients: &HashSet<String>,
) -> bool { 

    let total = ingredients.len();
    if total == 0 {
        panic!("Recipe is null");
    }

    let mut count = 0;
    for ing in ingredients {
        if liked_ingredients.contains(ing) {
            count += 1;
        }
    }

    let ratio = count as f32 / total as f32;
    
    // println!("Ratio was {}", ratio);

    return ratio >= 0.6;
}


#[test] // Person dislikes a recpe
fn testmefirst() -> Result<(), Box<dyn Error>> { 
    // println!("Current dir: {}", std::env::current_dir()?.display());
    let given_person = "Elliott Harding".to_lowercase();
    let given_recipe = "Recipe996".to_lowercase();
    let mut path = "./src/categories_ingredients.txt";
    let categories_ingredients = read_to_hashmap(path)?;
    path = "./src/people_categories.txt";
    let people_categories = read_to_hashmap(path)?;
    path = "./src/recipes.txt";
    let recipes = read_to_hashmap(path)?;

    let program_output = like_checker(&recipe_to_ingredients(&given_recipe, &recipes), &person_to_liked_ingredients(&given_person, &people_categories, &categories_ingredients));

    assert_eq!(program_output, false, "Specific person and recipe match is not working");
    Ok(())
}

#[test] // Person likes a recipe
fn testmesecond() -> Result<(), Box<dyn Error>> {
    // println!("Current dir: {}", std::env::current_dir()?.display()); 
    let given_person = "Halle Vaughn".to_lowercase();
    let given_recipe = "Recipe1".to_lowercase();
    let mut path = "./src/categories_ingredients.txt";
    let categories_ingredients = read_to_hashmap(path)?;
    path = "./src/people_categories.txt";
    let people_categories = read_to_hashmap(path)?;
    path = "./src/recipes.txt";
    let recipes = read_to_hashmap(path)?;

    let program_output = like_checker(&recipe_to_ingredients(&given_recipe, &recipes), &person_to_liked_ingredients(&given_person, &people_categories, &categories_ingredients));

    assert_eq!(program_output, true, "Specific person and recipe match is not working");
    Ok(())
}