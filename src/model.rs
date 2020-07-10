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

#[derive(Serialize, Deserialize, Debug)]
pub struct Resource {
    pub resource_type: ResourceType,
    pub source: Source,
    pub price: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Recipe {
    pub creates: Vec<(i32, ResourceType)>,
    pub consumes: Vec<(i32, ResourceType)>,
    pub time: i32,
}
