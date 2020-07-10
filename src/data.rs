use crate::model::*;
use prettytable::Table;

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
    raw_material_value: i32,
    ttm: i32,
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
            "Raw material value",
            "TTM",
        ]);

        for recipe in &self.recipes {
            let ev = self.evaluate_recipe(recipe);

            table.add_row(row![
                bill_as_string(&recipe.creates),
                format!("{}", ev.net_value),
                format!("{:.2}", ev.net_value_sec),
                format!("{}", ev.gross_value),
                format!("{}", ev.consumed_value),
                format!("{}", ev.raw_material_value),
                format!("{}", ev.ttm),
            ]);
        }

        table.printstd();
    }

    fn evaluate_recipe(&self, recipe: &Recipe) -> Evaluation {
        let mut consumed_value = 0;
        let mut gross_value = 0;
        let mut raw_material_value = 0;
        let mut ttm = recipe.time;

        for r in &recipe.creates {
            let res = self.get_resource(&r.1);
            gross_value += res.price * r.0;
        }

        for r in &recipe.consumes {
            let res = self.get_resource(&r.1);
            consumed_value += res.price * r.0;

            // Calculate values based on sub-recipes
            match self.get_recipe_for(&r.1) {
                Some(sub_recipe) => {
                    let sub_ev = self.evaluate_recipe(sub_recipe);
                    raw_material_value += sub_ev.raw_material_value * r.0;
                    ttm += sub_ev.ttm * r.0;
                }
                None => raw_material_value += res.price * r.0,
            };
        }

        let net_value = gross_value - consumed_value;
        let net_value_sec = net_value as f64 / recipe.time as f64;

        return Evaluation {
            net_value,
            net_value_sec,
            gross_value,
            consumed_value,
            raw_material_value,
            ttm,
        };
    }

    fn get_resource(&self, rtype: &ResourceType) -> &Resource {
        self.resources
            .iter()
            .find(|r| *rtype == r.resource_type)
            .unwrap()
    }

    fn get_recipe_for(&self, rtype: &ResourceType) -> Option<&Recipe> {
        self.recipes
            .iter()
            .find(|v| v.creates.iter().any(|r| *rtype == r.1))
    }
}

fn bill_as_string(bill: &Vec<(i32, ResourceType)>) -> String {
    return bill
        .iter()
        .map(|r| format!("{} {:?}", r.0, r.1))
        .collect::<Vec<String>>()
        .join(", ");
}
