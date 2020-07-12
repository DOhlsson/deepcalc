#[macro_use]
extern crate prettytable;

mod data;
mod model;

use data::Data;
use model::{Recipe, Resource};
use ron::de::from_reader;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let resources = load_data::<Resource>("data/resources.ron");
    let recipes = load_data::<Recipe>("data/recipes.ron");
    let data = Data::new(resources, recipes);

    println!("\nImported resources:");
    data.print_resources();

    println!("\nImported recipies:");
    data.evaluate_recipes();
}

fn load_data<T: serde::de::DeserializeOwned>(path: &str) -> Vec<T> {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    return match from_reader(reader) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("Error deserializing file {}: {}", path, e);
            std::process::exit(1);
        }
    };
}
