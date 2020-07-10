use prettytable::Table;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum ResourceType {
    Coal,
    Copper,
    Iron,
    Amber,
    Water,
    Aluminium,
    Silver,
    TreeSeed,
    Gold,
    CopperBar,
    IronBar,
    Silicon,
    Hydrogen,
    Glass,
    LabFlask,
    Oxygen,
    CleanWater,
    CopperNail,
    Graphite,
    Wire,
    Lamp,
    Battery,
    Circuits,
    Last, // Always last, could be used to calculate the length of this enum
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Source {
    Mine,
    WaterCollector,
    ChemicalMine,
    Chemistry,
    Smelt,
    Craft,
}

pub struct Data {
    resources: Vec<Resource>,
    recipes: Vec<Recipe>,
}

#[derive(Debug)]
pub struct Evaluation {
    net_value: i32,
    net_value_sec: f64,
    gross_value: i32,
    consumed_value: i32,
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

    pub fn print_resources(&self) {
        let mut table = Table::new();
        table.add_row(row!["Type", "Source", "Price",]);

        for resource in &self.resources {
            table.add_row(row![
                format!("{:?}", resource.resource_type),
                format!("{:?}", resource.source),
                resource.price,
            ]);
        }

        table.printstd();
    }

    pub fn evaluate_recipes(&self) {
        let mut table = Table::new();
        table.add_row(row![
            "Creates",
            "Net value",
            "Net value/s",
            "Gross value",
            "Consumed value",
        ]);

        for recipe in &self.recipes {
            let ev = self.evaluate_recipe(recipe);

            table.add_row(row![
                bill_as_string(&recipe.creates),
                format!("{}", ev.net_value),
                format!("{:.2}", ev.net_value_sec),
                format!("{}", ev.gross_value),
                format!("{}", ev.consumed_value),
            ]);
        }

        table.printstd();
    }

    fn evaluate_recipe(&self, recipe: &Recipe) -> Evaluation {
        let mut consumed_value = 0;
        let mut gross_value = 0;

        for r in &recipe.creates {
            let res = self.get_resource(&r.1);
            gross_value += res.price * r.0;
        }

        for r in &recipe.consumes {
            let res = self.get_resource(&r.1);
            consumed_value += res.price * r.0;
        }

        let net_value = gross_value - consumed_value;
        let net_value_sec = net_value as f64 / recipe.time as f64;

        return Evaluation {
            net_value,
            net_value_sec,
            gross_value,
            consumed_value,
        };
    }

    fn get_resource(&self, rtype: &ResourceType) -> &Resource {
        self.resources
            .iter()
            .find(|r| *rtype == r.resource_type)
            .unwrap()
    }
}

fn bill_as_string(bill: &Vec<(i32, ResourceType)>) -> String {
    return bill
        .iter()
        .map(|r| format!("{} {:?}", r.0, r.1))
        .collect::<Vec<String>>()
        .join(", ");
}
