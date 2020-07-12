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
    AluminiumBar,
    Silicon,
    Hydrogen,
    Glass,
    LabFlask,
    Oxygen,
    CleanWater,
    AluminiumBottle,
    CopperNail,
    InsulatedWire,
    Graphite,
    Wire,
    AmberInsulation,
    Lamp,
    AmberCharger,
    Battery,
    Circuits,
    Unknown,
    Last, // Always last, could be used to calculate the length of this enum
}

#[derive(Serialize, Deserialize, Debug)]
pub enum BuildingType {
    MiningStation,
    WaterCollector,
    ChemicalMine,
    Chemistry,
    Smelting,
    Crafting,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Resource {
    pub resource_type: ResourceType,
    pub price: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Recipe {
    pub creates: Vec<(i32, ResourceType)>,
    pub consumes: Vec<(i32, ResourceType)>,
    pub building: BuildingType,
    pub time: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Mine {
    pub area: i32,
    pub resources: Vec<(i32, ResourceType)>, // Resources by percent
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Building {
    pub building_type: BuildingType,
    pub level: i32,
    pub speed: i32,
    pub capacity: i32,
}
