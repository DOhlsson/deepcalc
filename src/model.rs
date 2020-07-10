use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum ResourceType {
    Coal,
    Graphite,
    Last, // Always last, used to calculate the length of this enum
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Source {
    Craft,
    Mine,
}

pub struct Data {
    resources: Vec<Resource>,
    recipes: Vec<Recipe>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Resource {
    resource_type: ResourceType,
    source: Source,
    price: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Recipe {
    creates: Vec<(i32, ResourceType)>,
    consumes: Vec<(i32, ResourceType)>,
    time: i32,
}

impl Data {
    pub fn new(resources: Vec<Resource>, recipes: Vec<Recipe>) -> Self {
        Data { resources, recipes }
    }

    pub fn evaluate_recipes(&self) {
        for recipe in &self.recipes {
            self.evaluate_recipe(recipe);
        }
    }

    fn evaluate_recipe(&self, recipe: &Recipe) {
        let mut created_value = 0;

        for r in &recipe.creates {
            let res = self.get_resource(&r.1);
            created_value += res.price * r.0;
        }

        for r in &recipe.consumes {
            let res = self.get_resource(&r.1);
            created_value -= res.price * r.0;
        }

        println!("{:?}", recipe);
        println!("  created_value: {}", created_value);
    }

    fn get_resource(&self, rtype: &ResourceType) -> &Resource {
        self.resources
            .iter()
            .find(|r| *rtype == r.resource_type)
            .unwrap()
    }
}
