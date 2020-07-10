#[macro_use]
extern crate prettytable;

mod model;

use model::*;
use ron::de::from_reader;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let resources = load_resources("data/resources.ron");
    let recipes = load_recipes("data/recipes.ron");
    let data = Data::new(resources, recipes);

    println!("\nImported resources:");
    data.print_resources();

    println!("\nImported recipies:");
    data.evaluate_recipes();
}

fn load_resources(path: &str) -> Vec<Resource> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    return match from_reader(reader) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("Error deserializing: {}", e);
            std::process::exit(1);
        }
    };
}

fn load_recipes(path: &str) -> Vec<Recipe> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    return match from_reader(reader) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("Error deserializing: {}", e);
            std::process::exit(1);
        }
    };
}
