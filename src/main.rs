use std::io;

mod cluster;
mod structure;
mod resource;
mod kind;

use cluster::Cluster;
use kind::Kind;

fn main() {
    let mut available = Cluster {
        name: "Available".to_string(),
        structures: Vec::new(),
    };
    let mut selected = Cluster {
        name: "Selected".to_string(),
        structures: Vec::new(),
    };

    available.add_structure(
        "Corridor", vec![
        (Kind::Money, -0.5),
        ]);
    available.add_structure(
        "Structured Corridor I", vec![
        (Kind::Money, -1.0),
        ]);
    available.add_structure(
        "Structured Corridor II", vec![
        (Kind::Money, -1.0),
        ]);
    available.add_structure(
        "Structured Corridor V", vec![
        (Kind::Money, -1.0),
        ]);
    available.add_structure(
        "Structured Corridor -V", vec![
        (Kind::Money, -1.0),
        ]);
    available.add_structure(
        "Elevator", vec![
        (Kind::Money, -1.0),
        (Kind::Electricity, -0.2),
        ]);
    available.add_structure(
        "Newsstand", vec![
        (Kind::Consumer, -1.0),
        (Kind::Labor, -1.0),
        (Kind::Money, 3.0),
        (Kind::Community, 2.0),
        (Kind::Knowledge, 1.0),
        ]);
    available.add_structure(
        "Newsstand", vec![
        (Kind::Consumer, -1.0),
        (Kind::Labor, -1.0),
        (Kind::Risk, -1.0),
        (Kind::Money, 3.0),
        (Kind::Community, 2.0),
        (Kind::Knowledge, 1.0),
        ]);
    available.add_structure(
        "Lamppost", vec![
        (Kind::Electricity, -1.0),
        (Kind::Water, -0.5),
        (Kind::Community, 1.0),
        (Kind::Leisure, 2.0),
        ]);
    available.add_structure(
        "Neon Ad 1", vec![
        (Kind::Electricity, -3.0),
        (Kind::Tourist, 1.0),
        (Kind::Consumer, 3.0),
        ]);
    available.add_structure(
        "Neon Ad 2", vec![
        (Kind::Electricity, -3.0),
        (Kind::Tourist, 1.0),
        (Kind::Consumer, 3.0),
        ]);
    available.add_structure(
        "Neon Ad 3", vec![
        (Kind::Electricity, -3.0),
        (Kind::Tourist, 1.0),
        (Kind::Consumer, 3.0),
        ]);
    available.add_structure(
        "Neon Ad 4", vec![
        (Kind::Electricity, -3.0),
        (Kind::Tourist, 1.0),
        (Kind::Consumer, 3.0),
        ]);
    available.add_structure(
        "Neon Ad 5", vec![
        (Kind::Electricity, -3.0),
        (Kind::Tourist, 1.0),
        (Kind::Consumer, 3.0),
        ]);
    available.add_structure(
        "Planters", vec![
        (Kind::Water, -1.0),
        (Kind::Community, -2.0),
        (Kind::Fertilizer, -1.0),
        (Kind::Vegetables, 4.0),
        ]);
    available.add_structure(
        "Playground", vec![
        (Kind::Youth, -2.0),
        (Kind::Community, 2.0),
        (Kind::Leisure, 4.0),
        ]);
    available.add_structure(
        "Plaza", vec![
        (Kind::Money, -1.0),
        (Kind::Community, 1.0),
        (Kind::Leisure, 2.0),
        ]);
    available.add_structure(
        "Sitting Area", vec![
        (Kind::Tourist, -2.0),
        (Kind::Community, 1.0),
        (Kind::Leisure, 2.0),
        (Kind::Consumer, 2.0),
        ]);
    available.add_structure(
        "Stairs", vec![
        (Kind::Money, -0.2),
        ]);
    available.add_structure(
        "Structure I", vec![
        (Kind::Money, -1.0),
        ]);
    available.add_structure(
        "Structure II", vec![
        (Kind::Money, -1.0),
        ]);
    available.add_structure(
        "Structure V", vec![
        (Kind::Money, -1.0),
        ]);
    available.add_structure(
        "Structure -V", vec![
        (Kind::Money, -1.0),
        ]);
    available.add_structure(
        "Escalator", vec![
        (Kind::Money, -0.5),
        (Kind::Electricity, -1.0),
        ]);
    available.add_structure(
        "Community Center", vec![
        (Kind::Electricity, -1.0),
        (Kind::Money, -1.0),
        (Kind::Community, 5.0),
        (Kind::SortedWaste, 2.0),
        ]);
    available.add_structure(
        "Community Center", vec![
        (Kind::Electricity, -1.0),
        (Kind::Money, -1.0),
        (Kind::InorganicWaste, -2.0),
        (Kind::Community, 5.0),
        (Kind::SortedWaste, 2.0),
        ]);
    available.add_structure(
        "Beer Garden", vec![
        (Kind::Beer, -2.0),
        (Kind::Consumer, -2.0),
        (Kind::FreshAir, -2.0),
        (Kind::Community, 1.0),
        (Kind::Leisure, 4.0),
        (Kind::Money, 8.0),
        ]);
    available.add_structure(
        "Recycling Plant", vec![
        (Kind::SortedWaste, -8.0),
        (Kind::Electricity, -1.0),
        (Kind::Money, 2.0),
        (Kind::Plastics, 4.0),
        (Kind::Fabric, 4.0),
        ]);
    available.add_structure(
        "Waste Sorting Center", vec![
        (Kind::InorganicWaste, -8.0),
        (Kind::OrganicWaste, -8.0),
        (Kind::Electricity, -1.0),
        (Kind::Labor, -1.0),
        (Kind::SortedWaste, 8.0),
        ]);
    available.add_structure(
        "Cemetery", vec![
        (Kind::Labor, -1.0),
        (Kind::Community, 1.0),
        ]);
    available.add_structure(
        "Cemetery", vec![
        (Kind::Labor, -1.0),
        (Kind::Sickness, -3.0),
        (Kind::Community, 1.0),
        ]);
    available.add_structure(
        "Vegetable Stand", vec![
        (Kind::Vegetables, -2.0),
        (Kind::Consumer, -1.0),
        (Kind::Community, 4.0),
        (Kind::Money, 4.0),
        ]);
    available.add_structure(
        "Cheese Stand", vec![
        (Kind::Milk, -2.0),
        (Kind::Consumer, -1.0),
        (Kind::Community, 4.0),
        (Kind::Money, 4.0),
        ]);
    available.add_structure(
        "Fish Stand", vec![
        (Kind::Fish, -2.0),
        (Kind::Consumer, -1.0),
        (Kind::Community, 4.0),
        (Kind::Money, 4.0),
        ]);
    available.add_structure(
        "Fruit Stand", vec![
        (Kind::Fruit, -2.0),
        (Kind::Consumer, -1.0),
        (Kind::Community, 4.0),
        (Kind::Money, 4.0),
        ]);
    available.add_structure(
        "Solar Tree", vec![
        (Kind::Money, -2.0),
        (Kind::Electricity, 1.0),
        (Kind::Community, 1.0),
        (Kind::Leisure, 1.0),
        ]);
    available.add_structure(
        "Food Truck", vec![
        (Kind::Labor, -1.0),
        (Kind::Biogas, -1.0),
        (Kind::Meat, -2.0),
        (Kind::Corn, -2.0),
        (Kind::Money, 3.0),
        (Kind::Community, 2.0),
        (Kind::GreenhouseGas, 1.0),
        ]);
    available.add_structure(
        "Basketball Court", vec![
        (Kind::Community, -3.0),
        (Kind::Leisure, 1.0),
        (Kind::Fitness, 1.0),
        (Kind::Entertainment, 1.0),
        ]);
    available.add_structure(
        "Football Pitch", vec![
        (Kind::Community, -3.0),
        (Kind::Leisure, 1.0),
        (Kind::Fitness, 1.0),
        (Kind::Entertainment, 1.0),
        ]);
    available.add_structure(
        "Runway Stage", vec![
        (Kind::Garment, -2.0),
        (Kind::Performer, -2.0),
        (Kind::Culture, -1.0),
        (Kind::Consumer, 6.0),
        (Kind::Entertainment, 2.0),
        (Kind::Money, 1.0),
        ]);
    available.add_structure(
        "Amphitheater", vec![
        (Kind::FreshAir, -2.0),
        (Kind::Community, -2.0),
        (Kind::Entertainment, 1.0),
        (Kind::Culture, 1.0),
        (Kind::Leisure, 4.0),
        ]);
    available.add_structure(
        "Amphitheater", vec![
        (Kind::FreshAir, -2.0),
        (Kind::Community, -2.0),
        (Kind::Performer, -2.0),
        (Kind::Entertainment, 1.0),
        (Kind::Culture, 1.0),
        (Kind::Leisure, 4.0),
        ]);
    available.add_structure(
        "Bike Station", vec![
        (Kind::Electricity, -1.0),
        (Kind::Community, -2.0),
        (Kind::Consumer, 2.0),
        (Kind::Fitness, 2.0),
        (Kind::Money, 2.0),
        ]);
    available.add_structure(
        "Botanical Garden", vec![
        (Kind::Water, -3.0),
        (Kind::Labor, -1.0),
        (Kind::Fertilizer, -2.0),
        (Kind::FreshAir, 3.0),
        (Kind::Leisure, 2.0),
        (Kind::Community, 2.0),
        (Kind::Tourist, 2.0),
        ]);
    available.add_structure(
        "Botanical Garden", vec![
        (Kind::Water, -3.0),
        (Kind::Labor, -1.0),
        (Kind::Fertilizer, -2.0),
        (Kind::Wilderness, -3.0),
        (Kind::FreshAir, 3.0),
        (Kind::Leisure, 2.0),
        (Kind::Community, 2.0),
        (Kind::Tourist, 2.0),
        ]);
    available.add_structure(
        "Green Wall", vec![
        (Kind::Water, -1.0),
        (Kind::FreshAir, 1.0),
        (Kind::Leisure, 1.0),
        ]);
    available.add_structure(
        "Obelisk", vec![
        (Kind::Money, -3.0),
        (Kind::Community, 1.0),
        (Kind::Culture, 1.0),
        (Kind::Leisure, 2.0),
        (Kind::Tourist, 2.0),
        ]);
    available.add_structure(
        "Torii Arch", vec![
        (Kind::Money, -2.0),
        (Kind::Community, 1.0),
        (Kind::Culture, 2.0),
        (Kind::Tourist, 2.0),
        ]);
    available.add_structure(
        "Spa", vec![
        (Kind::HotWater, -4.0),
        (Kind::Consumer, -4.0),
        (Kind::Labor, -2.0),
        (Kind::Money, 12.0),
        (Kind::Greywater, 4.0),
        ]);
    available.add_structure(
        "Hybrid Park I", vec![
        (Kind::Money, -2.0),
        (Kind::Water, -2.0),
        (Kind::FreshAir, 1.0),
        (Kind::Leisure, 1.0),
        (Kind::Fitness, 1.0),
        (Kind::Community, 1.0),
        ]);
    available.add_structure(
        "Hybrid Park II", vec![
        (Kind::Money, -2.0),
        (Kind::Water, -2.0),
        (Kind::FreshAir, 1.0),
        (Kind::Leisure, 1.0),
        (Kind::Fitness, 1.0),
        (Kind::Community, 1.0),
        ]);
    available.add_structure(
        "Hybrid Park III", vec![
        (Kind::Money, -2.0),
        (Kind::Water, -2.0),
        (Kind::FreshAir, 1.0),
        (Kind::Leisure, 1.0),
        (Kind::Fitness, 1.0),
        (Kind::Community, 1.0),
        ]);
    available.add_structure(
        "Hybrid Park IV", vec![
        (Kind::Money, -2.0),
        (Kind::Water, -2.0),
        (Kind::FreshAir, 1.0),
        (Kind::Leisure, 1.0),
        (Kind::Fitness, 1.0),
        (Kind::Community, 1.0),
        ]);
    available.add_structure(
        "Zen Garden", vec![
        (Kind::Groundwater, -1.0),
        (Kind::Culture, -1.0),
        (Kind::Leisure, 2.0),
        (Kind::FreshAir, 1.0),
        (Kind::Community, 3.0),
        ]);

    // Production blocks

    available.add_structure(
        "Algae Farm", vec![
        (Kind::Greywater, -8.0),
        (Kind::Algae, 4.0),
        (Kind::Water, 8.0),
        ]);
    available.add_structure(
        "Algae Farm", vec![
        (Kind::Greywater, -8.0),
        (Kind::GreenhouseGas, -1.0),
        (Kind::Algae, 4.0),
        (Kind::Water, 8.0),
        ]);
    available.add_structure(
        "Anaerobic Digestor", vec![
        (Kind::OrganicWaste, -6.0),
        (Kind::Water, -3.0),
        (Kind::Biogas, 4.0),
        (Kind::Fertilizer, 4.0),
        ]);
    available.add_structure(
        "Bee Farm", vec![
        (Kind::Money, -1.0),
        (Kind::Labor, -1.0),
        (Kind::Bees, 3.0),
        ]);
    available.add_structure(
        "Biomass Generator", vec![
        (Kind::Algae, -10.0),
        (Kind::Electricity, 12.0),
        (Kind::Fertilizer, 1.0),
        ]);
    available.add_structure(
        "Fish Farm", vec![
        (Kind::Water, -2.0),
        (Kind::OrganicWaste, -4.0),
        (Kind::Labor, -1.0),
        (Kind::Fish, 4.0),
        (Kind::Greywater, 2.0),
        (Kind::Fertilizer, 2.0),
        ]);
    available.add_structure(
        "Hydroponic Farm", vec![
        (Kind::Water, -1.0),
        (Kind::Fertilizer, -0.5),
        (Kind::Electricity, -1.0),
        (Kind::Vegetables, 4.0),
        (Kind::Greywater, 1.0),
        ]);
    available.add_structure(
        "Compost Plant", vec![
        (Kind::OrganicWaste, -4.0),
        (Kind::Labor, -1.0),
        (Kind::FreshAir, -2.0),
        (Kind::Fertilizer, 4.0),
        (Kind::GreenhouseGas, 0.25),
        ]);
    available.add_structure(
        "Landfill", vec![
        (Kind::InorganicWaste, -4.0),
        (Kind::OrganicWaste, -4.0),
        (Kind::Labor, -1.0),
        (Kind::GreenhouseGas, 4.0),
        (Kind::Sickness, 1.0),
        ]);
    available.add_structure(
        "Landfill", vec![
        (Kind::InorganicWaste, -4.0),
        (Kind::OrganicWaste, -4.0),
        (Kind::Labor, -1.0),
        (Kind::FreshAir, -3.0),
        (Kind::GreenhouseGas, 4.0),
        (Kind::Sickness, 1.0),
        ]);
    available.add_structure(
        "Oil Well", vec![
        (Kind::Money, -1.0),
        (Kind::Oil, 4.0),
        (Kind::InorganicWaste, 4.0),
        (Kind::GreenhouseGas, 1.0),
        ]);
    available.add_structure(
        "Oil Tank", vec![
        (Kind::Oil, -4.0),
        (Kind::Oil, 2.0),
        (Kind::Money, 8.0),
        (Kind::GreenhouseGas, 0.5),
        ]);
    available.add_structure(
        "Solar Panel", vec![
        (Kind::Electricity, 1.0),
        ]);
    available.add_structure(
        "Vegetable Farm", vec![
        (Kind::Groundwater, -1.0),
        (Kind::Labor, -2.0),
        (Kind::Bees, -1.0),
        (Kind::Fertilizer, -1.0),
        (Kind::Vegetables, 6.0),
        (Kind::GreenhouseGas, 2.0),
        ]);
    available.add_structure(
        "Water Tower", vec![
        (Kind::Money, -2.0),
        (Kind::Water, 8.0),
        ]);
    available.add_structure(
        "Water Well", vec![
        (Kind::Groundwater, -2.0),
        (Kind::Water, 2.0),
        ]);
    available.add_structure(
        "Windmill", vec![
        (Kind::Money, -1.0),
        (Kind::Electricity, 2.0),
        ]);
    available.add_structure(
        "Oil Generator", vec![
        (Kind::Oil, -2.0),
        (Kind::Money, -1.0),
        (Kind::Electricity, 20.0),
        (Kind::GreenhouseGas, 6.0),
        (Kind::Sickness, 4.0),
        ]);
    available.add_structure(
        "Gas Generator", vec![
        (Kind::Biogas, -4.0),
        (Kind::Money, -1.0),
        (Kind::Electricity, 10.0),
        (Kind::Pollution, 1.0),
        (Kind::GreenhouseGas, 3.0),
        ]);
    available.add_structure(
        "Incinerator", vec![
        (Kind::Electricity, -1.0),
        (Kind::InorganicWaste, -6.0),
        (Kind::Electricity, 4.0),
        (Kind::GreenhouseGas, 4.0),
        (Kind::Pollution, 2.0),
        ]);
    available.add_structure(
        "Boiler", vec![
        (Kind::Water, -6.0),
        (Kind::Oil, -1.0),
        (Kind::HotWater, 6.0),
        (Kind::Pollution, 0.5),
        ]);
    available.add_structure(
        "Fog Collector", vec![
        (Kind::Money, -0.5),
        (Kind::Knowledge, -0.5),
        (Kind::Water, 4.0),
        ]);
    available.add_structure(
        "Solar Heater", vec![
        (Kind::Water, -4.0),
        (Kind::HotWater, 4.0),
        ]);
    available.add_structure(
        "Rice Field", vec![
        (Kind::Groundwater, -2.0),
        (Kind::Labor, -2.0),
        (Kind::Fertilizer, -1.0),
        (Kind::Rice, 4.0),
        (Kind::GreenhouseGas, 1.0),
        ]);
    available.add_structure(
        "Geothermal Generator", vec![
        (Kind::Money, -3.0),
        (Kind::Electricity, 9.0),
        (Kind::GreenhouseGas, 0.1),
        ]);
    available.add_structure(
        "Sprinklers", vec![
        (Kind::Electricity, -0.5),
        (Kind::Water, -1.0),
        (Kind::Groundwater, 2.0),
        ]);
    available.add_structure(
        "Brewery", vec![
        (Kind::Wheat, -2.0),
        (Kind::Water, -2.0),
        (Kind::Labor, -2.0),
        (Kind::Beer, 4.0),
        (Kind::OrganicWaste, 2.0),
        ]);
    available.add_structure(
        "Wheat Field", vec![
        (Kind::Groundwater, -1.0),
        (Kind::Labor, -2.0),
        (Kind::Fertilizer, -1.0),
        (Kind::Wheat, 2.0),
        (Kind::GreenhouseGas, 2.0),
        ]);
    available.add_structure(
        "Pig Pen", vec![
        (Kind::Water, -1.0),
        (Kind::Labor, -1.0),
        (Kind::Vegetables, -1.0),
        (Kind::Corn, -1.0),
        (Kind::Meat, 2.0),
        (Kind::OrganicWaste, 2.0),
        (Kind::GreenhouseGas, 2.0),
        ]);
    available.add_structure(
        "Cow Pasture", vec![
        (Kind::Water, -2.0),
        (Kind::Labor, -1.0),
        (Kind::Wheat, -1.0),
        (Kind::Meat, 2.0),
        (Kind::OrganicWaste, 1.0),
        (Kind::Milk, 2.0),
        (Kind::GreenhouseGas, 2.0),
        ]);
    available.add_structure(
        "Corn Field", vec![
        (Kind::Groundwater, -1.0),
        (Kind::Labor, -1.0),
        (Kind::Fertilizer, -1.0),
        (Kind::Corn, 3.0),
        (Kind::GreenhouseGas, 1.0),
        ]);
    available.add_structure(
        "Vineyard", vec![
        (Kind::Groundwater, -1.0),
        (Kind::Fertilizer, -2.0),
        (Kind::Labor, -1.0),
        (Kind::Fruit, 4.0),
        (Kind::GreenhouseGas, 1.0),
        (Kind::OrganicWaste, 1.0),
        ]);
    available.add_structure(
        "Grain Silo", vec![
        (Kind::Corn, -4.0),
        (Kind::Corn, 2.0),
        (Kind::Money, 8.0),
        ]);
    available.add_structure(
        "Wine Cellar", vec![
        (Kind::Juice, -6.0),
        (Kind::Wine, 5.0),
        ]);
    available.add_structure(
        "Fermenter", vec![
        (Kind::Fruit, -4.0),
        (Kind::Electricity, -1.0),
        (Kind::Juice, 4.0),
        (Kind::OrganicWaste, 1.0),
        ]);
    available.add_structure(
        "Chicken Barn", vec![
        (Kind::Corn, -1.0),
        (Kind::Wheat, -1.0),
        (Kind::Labor, -1.0),
        (Kind::Meat, 2.0),
        (Kind::Eggs, 2.0),
        (Kind::GreenhouseGas, 1.0),
        ]);
    available.add_structure(
        "Sheep Pasture", vec![
        (Kind::Water, -2.0),
        (Kind::Labor, -1.0),
        (Kind::Wheat, -1.0),
        (Kind::Meat, 2.0),
        (Kind::Milk, 1.0),
        (Kind::OrganicWaste, 1.0),
        (Kind::Wool, 1.0),
        ]);
    available.add_structure(
        "Cow Cafo", vec![
        (Kind::Water, -2.0),
        (Kind::Corn, -4.0),
        (Kind::Meat, 8.0),
        (Kind::Sickness, 2.0),
        (Kind::Milk, 2.0),
        (Kind::GreenhouseGas, 4.0),
        ]);
    available.add_structure(
        "GMO Corn Field", vec![
        (Kind::Groundwater, -1.0),
        (Kind::Labor, -1.0),
        (Kind::Fertilizer, -1.0),
        (Kind::Pesticides, -1.0),
        (Kind::Corn, 5.0),
        (Kind::Sickness, 2.0),
        (Kind::GreenhouseGas, 2.0),
        ]);
    available.add_structure(
        "Pesticide Shack", vec![
        (Kind::Money, -1.0),
        (Kind::Labor, -1.0),
        (Kind::Pesticides, 4.0),
        (Kind::Sickness, 1.0),
        ]);
    available.add_structure(
        "Ethanol Distillery", vec![
        (Kind::Corn, -6.0),
        (Kind::Water, -4.0),
        (Kind::Electricity, -2.0),
        (Kind::Ethanol, 8.0),
        (Kind::OrganicWaste, 4.0),
        ]);
    available.add_structure(
        "Ethanol Generator", vec![
        (Kind::Ethanol, -8.0),
        (Kind::Money, -2.0),
        (Kind::Electricity, 10.0),
        ]);
    available.add_structure(
        "Cotton Field", vec![
        (Kind::Groundwater, -0.75),
        (Kind::Fertilizer, -1.0),
        (Kind::Labor, -2.0),
        (Kind::Pesticides, -1.0),
        (Kind::RawCotton, 4.0),
        (Kind::OrganicWaste, 1.0),
        (Kind::GreenhouseGas, 1.0),
        ]);
    available.add_structure(
        "Industrial Gin", vec![
        (Kind::RawCotton, -8.0),
        (Kind::Electricity, -2.0),
        (Kind::CottonBale, 4.0),
        (Kind::OrganicWaste, 2.0),
        ]);
    available.add_structure(
        "Cotton Mill", vec![
        (Kind::CottonBale, -4.0),
        (Kind::Electricity, -2.0),
        (Kind::Labor, -2.0),
        (Kind::Fabric, 10.0),
        (Kind::InorganicWaste, 2.0),
        ]);
    available.add_structure(
        "Wool Mill", vec![
        (Kind::Wool, -4.0),
        (Kind::Electricity, -2.0),
        (Kind::Labor, -1.0),
        (Kind::Fabric, 8.0),
        (Kind::InorganicWaste, 2.0),
        ]);
    available.add_structure(
        "Ostrich Farm", vec![
        (Kind::Water, -1.0),
        (Kind::Wheat, -1.0),
        (Kind::Corn, -1.0),
        (Kind::Labor, -1.0),
        (Kind::Meat, 2.0),
        (Kind::Eggs, 1.0),
        (Kind::Leather, 2.0),
        (Kind::OrganicWaste, 2.0),
        ]);
    available.add_structure(
        "Water Treatment Plant", vec![
        (Kind::Electricity, -2.0),
        (Kind::Saltwater, -4.0),
        (Kind::Water, 8.0),
        ]);
    available.add_structure(
        "Water Treatment Plant", vec![
        (Kind::Electricity, -2.0),
        (Kind::Saltwater, -4.0),
        (Kind::Greywater, -4.0),
        (Kind::Water, 8.0),
        ]);
    available.add_structure(
        "Dry Coffee Processing", vec![
        (Kind::CoffeeCherry, -4.0),
        (Kind::Labor, -2.0),
        (Kind::FreshAir, -2.0),
        (Kind::CoffeeBeans, 4.0),
        (Kind::OrganicWaste, 1.0),
        ]);
    available.add_structure(
        "Wet Coffee Process", vec![
        (Kind::CoffeeCherry, -4.0),
        (Kind::Labor, -1.0),
        (Kind::Water, -2.0),
        (Kind::CoffeeBeans, 4.0),
        (Kind::Greywater, 1.0),
        (Kind::OrganicWaste, 1.0),
        ]);

    // Building blocks

    available.add_structure(
        "Bakery", vec![
        (Kind::Wheat, -6.0),
        (Kind::Labor, -1.0),
        (Kind::Consumer, -2.0),
        (Kind::Bread, 4.0),
        (Kind::Money, 4.0),
        (Kind::OrganicWaste, 1.0),
        ]);
    available.add_structure(
        "Bar", vec![
        (Kind::Beer, -3.0),
        (Kind::Labor, -2.0),
        (Kind::Consumer, -4.0),
        (Kind::Community, 2.0),
        (Kind::Money, 12.0),
        ]);
    available.add_structure(
        "Cafe", vec![
        (Kind::Consumer, -4.0),
        (Kind::Labor, -1.0),
        (Kind::Bread, -2.0),
        (Kind::Money, 8.0),
        (Kind::Community, 2.0),
        (Kind::OrganicWaste, 2.0),
        ]);
    available.add_structure(
        "Cafe", vec![
        (Kind::Consumer, -4.0),
        (Kind::Labor, -1.0),
        (Kind::Bread, -2.0),
        (Kind::RoastedCoffee, -2.0),
        (Kind::Money, 8.0),
        (Kind::Community, 2.0),
        (Kind::OrganicWaste, 2.0),
        ]);
    available.add_structure(
        "Clinic", vec![
        (Kind::Electricity, -1.0),
        (Kind::Labor, -1.0),
        (Kind::Knowledge, -1.0),
        (Kind::Plastics, -1.0),
        (Kind::Community, 2.0),
        (Kind::Sickness, -5.0),
        (Kind::Fitness, 2.0),
        ]);
    available.add_structure(
        "Corner Store", vec![
        (Kind::Labor, -1.0),
        (Kind::Electricity, -1.0),
        (Kind::Consumer, -4.0),
        (Kind::Money, 6.0),
        (Kind::Community, 1.0),
        (Kind::InorganicWaste, 2.0),
        ]);
    available.add_structure(
        "Plastics Factory", vec![
        (Kind::Labor, -4.0),
        (Kind::Electricity, -1.0),
        (Kind::Oil, -2.0),
        (Kind::Money, 12.0),
        (Kind::Plastics, 6.0),
        (Kind::InorganicWaste, 4.0),
        (Kind::GreenhouseGas, 3.0),
        ]);
    available.add_structure(
        "Capsule Hotel", vec![
        (Kind::Electricity, -1.0),
        (Kind::HotWater, -1.0),
        (Kind::Tourist, -1.0),
        (Kind::Money, 7.0),
        (Kind::Consumer, 4.0),
        (Kind::OrganicWaste, 1.0),
        (Kind::Greywater, 1.0),
        ]);
    available.add_structure(
        "Corner Apt", vec![
        (Kind::FreshAir, -1.0),
        (Kind::Electricity, -1.0),
        (Kind::Water, -1.0),
        (Kind::Leisure, -2.0),
        (Kind::Labor, 2.0),
        (Kind::Greywater, 1.0),
        (Kind::OrganicWaste, 2.0),
        (Kind::Youth, 2.0),
        ]);
    available.add_structure(
        "Large Apt", vec![
        (Kind::FreshAir, -1.0),
        (Kind::Electricity, -1.0),
        (Kind::Water, -2.0),
        (Kind::Leisure, -3.0),
        (Kind::Youth, 3.0),
        (Kind::Greywater, 3.0),
        (Kind::OrganicWaste, 3.0),
        (Kind::Consumer, 5.0),
        ]);
    available.add_structure(
        "Small Apt", vec![
        (Kind::FreshAir, -1.0),
        (Kind::Electricity, -1.0),
        (Kind::Water, -1.0),
        (Kind::Leisure, -1.0),
        (Kind::Labor, 3.0),
        (Kind::Greywater, 2.0),
        (Kind::OrganicWaste, 1.0),
        (Kind::Consumer, 2.0),
        ]);
    available.add_structure(
        "Market", vec![
        (Kind::Electricity, -1.0),
        (Kind::Consumer, -4.0),
        (Kind::Vegetables, -3.0),
        (Kind::Fish, -3.0),
        (Kind::Money, 10.0),
        (Kind::Community, 1.0),
        ]);
    available.add_structure(
        "Office", vec![
        (Kind::Electricity, -1.0),
        (Kind::Labor, -2.0),
        (Kind::Knowledge, -1.0),
        (Kind::Money, 6.0),
        (Kind::Consumer, 2.0),
        ]);
    available.add_structure(
        "Office", vec![
        (Kind::Electricity, -1.0),
        (Kind::Labor, -2.0),
        (Kind::Knowledge, -1.0),
        (Kind::RoastedCoffee, -1.0),
        (Kind::Money, 6.0),
        (Kind::Consumer, 2.0),
        ]);
    available.add_structure(
        "School", vec![
        (Kind::Labor, -1.0),
        (Kind::Youth, -8.0),
        (Kind::Knowledge, 6.0),
        (Kind::Community, 2.0),
        ]);
    available.add_structure(
        "Greenhouse", vec![
        (Kind::Water, -1.0),
        (Kind::Labor, -2.0),
        (Kind::Fertilizer, -3.0),
        (Kind::Vegetables, 4.0),
        (Kind::Electricity, 1.0),
        ]);
    available.add_structure(
        "Corp. Office", vec![
        (Kind::Oil, -1.0),
        (Kind::Knowledge, -1.0),
        (Kind::Labor, -3.0),
        (Kind::Electricity, -2.0),
        (Kind::Money, 12.0),
        (Kind::InorganicWaste, 2.0),
        (Kind::RoastedCoffee, -1.5),
        ]);
    available.add_structure(
        "Hotel Room", vec![
        (Kind::HotWater, -2.0),
        (Kind::Electricity, -2.0),
        (Kind::Tourist, -2.0),
        (Kind::FreshAir, -2.0),
        (Kind::Money, 10.0),
        (Kind::Consumer, 6.0),
        (Kind::Greywater, 3.0),
        (Kind::OrganicWaste, 3.0),
        ]);
    available.add_structure(
        "Art Gallery", vec![
        (Kind::Electricity, -1.0),
        (Kind::Labor, -1.0),
        (Kind::Consumer, -1.0),
        (Kind::Community, -4.0),
        (Kind::Culture, 4.0),
        (Kind::Tourist, 8.0),
        ]);
    available.add_structure(
        "Coffee Shop", vec![
        (Kind::HotWater, -2.0),
        (Kind::CoffeeBeans, -3.0),
        (Kind::Consumer, -2.0),
        (Kind::Labor, -1.0),
        (Kind::Money, 6.0),
        (Kind::RoastedCoffee, 3.0),
        (Kind::OrganicWaste, 1.5),
        (Kind::Greywater, 1.5),
        ]);
    available.add_structure(
        "Makerspace", vec![
        (Kind::Knowledge, -1.0),
        (Kind::Youth, -4.0),
        (Kind::Plastics, -1.0),
        (Kind::Electricity, -1.0),
        (Kind::Technology, 3.0),
        (Kind::Community, 1.0),
        (Kind::InorganicWaste, 1.0),
        ]);
    available.add_structure(
        "Tech Office", vec![
        (Kind::Data, -2.0),
        (Kind::Electricity, -1.5),
        (Kind::Labor, -2.0),
        (Kind::Technology, 4.0),
        (Kind::Money, 4.0),
        ]);
    available.add_structure(
        "Tech Office", vec![
        (Kind::Data, -2.0),
        (Kind::Electricity, -1.5),
        (Kind::Labor, -2.0),
        (Kind::RoastedCoffee, -1.0),
        (Kind::Technology, 4.0),
        (Kind::Money, 4.0),
        ]);
    available.add_structure(
        "Tech Factory", vec![
        (Kind::Labor, -1.0),
        (Kind::Technology, -2.0),
        (Kind::Electricity, -2.0),
        (Kind::Plastics, -2.0),
        (Kind::Money, 8.0),
        (Kind::InorganicWaste, 1.5),
        (Kind::Devices, 4.0),
        ]);
    available.add_structure(
        "Library", vec![
        (Kind::Community, -2.0),
        (Kind::Electricity, -0.25),
        (Kind::Knowledge, 4.0),
        ]);
    available.add_structure(
        "Barbershop", vec![
        (Kind::Electricity, -1.0),
        (Kind::Labor, -1.0),
        (Kind::Consumer, -2.0),
        (Kind::Money, 4.0),
        (Kind::Community, 2.0),
        (Kind::OrganicWaste, 2.0),
        ]);
    available.add_structure(
        "Barbershop", vec![
        (Kind::Electricity, -1.0),
        (Kind::Labor, -1.0),
        (Kind::Consumer, -2.0),
        (Kind::Risk, -4.0),
        (Kind::Money, 4.0),
        (Kind::Community, 2.0),
        (Kind::OrganicWaste, 2.0),
        ]);
    available.add_structure(
        "Diner", vec![
        (Kind::Electricity, -2.0),
        (Kind::Consumer, -4.0),
        (Kind::Eggs, -2.0),
        (Kind::Money, 7.0),
        (Kind::OrganicWaste, 2.0),
        (Kind::Community, 3.0),
        (Kind::Risk, -2.0),
        ]);
    available.add_structure(
        "Diner", vec![
        (Kind::Electricity, -2.0),
        (Kind::Consumer, -4.0),
        (Kind::Eggs, -2.0),
        (Kind::RoastedCoffee, -2.0),
        (Kind::Money, 7.0),
        (Kind::OrganicWaste, 2.0),
        (Kind::Community, 3.0),
        (Kind::Risk, -2.0),
        ]);
    available.add_structure(
        "Antenna", vec![
        (Kind::Electricity, -2.0),
        (Kind::Technology, -1.0),
        (Kind::Data, 2.0),
        ]);
    available.add_structure(
        "Butcher Shop", vec![
        (Kind::Labor, -1.0),
        (Kind::Meat, -2.0),
        (Kind::Money, 6.0),
        (Kind::OrganicWaste, 1.0),
        ]);
    available.add_structure(
        "Fine Restaurant", vec![
        (Kind::Consumer, -4.0),
        (Kind::Labor, -2.0),
        (Kind::Meat, -4.0),
        (Kind::Wine, -4.0),
        (Kind::Money, 10.0),
        (Kind::Culture, 4.0),
        (Kind::OrganicWaste, 4.0),
        ]);
    available.add_structure(
        "Shack I", vec![
        (Kind::Water, -1.0),
        (Kind::Community, -1.0),
        (Kind::Labor, 5.0),
        (Kind::Risk, 2.0),
        (Kind::OrganicWaste, 4.0),
        (Kind::Greywater, 4.0),
        ]);
    available.add_structure(
        "Shack II", vec![
        (Kind::Water, -1.0),
        (Kind::Community, -1.0),
        (Kind::Labor, 5.0),
        (Kind::Risk, 2.0),
        (Kind::OrganicWaste, 4.0),
        (Kind::Greywater, 4.0),
        ]);
    available.add_structure(
        "Shack III", vec![
        (Kind::Water, -1.0),
        (Kind::Community, -1.0),
        (Kind::Labor, 5.0),
        (Kind::Risk, 2.0),
        (Kind::OrganicWaste, 4.0),
        (Kind::Greywater, 4.0),
        ]);
    available.add_structure(
        "Garment Sweatshop", vec![
        (Kind::Fabric, -10.0),
        (Kind::Labor, -4.0),
        (Kind::Electricity, -2.0),
        (Kind::Garment, 8.0),
        (Kind::InorganicWaste, 2.0),
        (Kind::Risk, 8.0),
        ]);
    available.add_structure(
        "Clothing Retailer", vec![
        (Kind::Electricity, -1.0),
        (Kind::Consumer, -3.0),
        (Kind::Labor, -1.0),
        (Kind::Garment, -6.0),
        (Kind::Money, 12.0),
        (Kind::InorganicWaste, 3.0),
        ]);
    available.add_structure(
        "Studio Space", vec![
        (Kind::Electricity, -1.0),
        (Kind::Money, -1.0),
        (Kind::Labor, -2.0),
        (Kind::Performer, 2.0),
        (Kind::Culture, 2.0),
        (Kind::Media, 2.0),
        ]);
    available.add_structure(
        "Data Center", vec![
        (Kind::Electricity, -2.0),
        (Kind::Money, -1.0),
        (Kind::Labor, -0.5),
        (Kind::Data, 4.0),
        ]);
    available.add_structure(
        "Processed Food Factory", vec![
        (Kind::Electricity, -2.0),
        (Kind::Labor, -1.0),
        (Kind::Meat, -3.0),
        (Kind::Corn, -3.0),
        (Kind::ProcessedFood, 6.0),
        (Kind::OrganicWaste, 2.0),
        ]);
    available.add_structure(
        "Fast Food Restaurant", vec![
        (Kind::Electricity, -2.0),
        (Kind::Labor, -1.0),
        (Kind::ProcessedFood, -6.0),
        (Kind::Consumer, -4.0),
        (Kind::Money, 8.0),
        (Kind::InorganicWaste, 2.0),
        (Kind::OrganicWaste, 2.0),
        ]);
    available.add_structure(
        "Internet Cafe", vec![
        (Kind::Electricity, -2.0),
        (Kind::Consumer, -4.0),
        (Kind::Data, -4.0),
        (Kind::Devices, -1.0),
        (Kind::Money, 8.0),
        (Kind::Community, 2.0),
        (Kind::Entertainment, 2.0),
        ]);
    available.add_structure(
        "Jazz Club", vec![
        (Kind::Electricity, -1.0),
        (Kind::Performer, -2.0),
        (Kind::Consumer, -6.0),
        (Kind::Money, 7.0),
        (Kind::Entertainment, 3.0),
        (Kind::Culture, 3.0),
        ]);
    available.add_structure(
        "Luxury Apartment", vec![
        (Kind::Electricity, -2.0),
        (Kind::HotWater, -2.0),
        (Kind::FreshAir, -1.0),
        (Kind::Technology, -1.0),
        (Kind::Labor, 1.0),
        (Kind::Consumer, 4.0),
        (Kind::Youth, 2.0),
        (Kind::SortedWaste, 4.0),
        ]);
    available.add_structure(
        "Luxury Loft", vec![
        (Kind::Electricity, -5.0),
        (Kind::HotWater, -5.0),
        (Kind::FreshAir, -2.0),
        (Kind::Labor, -1.0),
        (Kind::Consumer, 20.0),
        (Kind::Youth, 3.0),
        (Kind::SortedWaste, 3.0),
        (Kind::Greywater, 3.0),
        ]);
    available.add_structure(
        "Mediatheque", vec![
        (Kind::Electricity, -2.0),
        (Kind::Community, -2.0),
        (Kind::Data, -2.0),
        (Kind::Media, -2.0),
        (Kind::Knowledge, 4.0),
        (Kind::Culture, 2.0),
        (Kind::Entertainment, 2.0),
        ]);
    available.add_structure(
        "Noodle Stall", vec![
        (Kind::HotWater, -3.0),
        (Kind::Eggs, -3.0),
        (Kind::Wheat, -1.0),
        (Kind::Consumer, -3.0),
        (Kind::Money, 7.0),
        (Kind::OrganicWaste, 2.0),
        (Kind::Community, 1.0),
        ]);
    available.add_structure(
        "Shipping Center", vec![
        (Kind::Electricity, -2.0),
        (Kind::Plastics, -2.0),
        (Kind::Labor, -2.0),
        (Kind::Oil, -4.0),
        (Kind::Money, 11.0),
        (Kind::InorganicWaste, 4.0),
        (Kind::GreenhouseGas, 3.0),
        ]);
    available.add_structure(
        "Sushi Bar", vec![
        (Kind::Rice, -4.0),
        (Kind::Fish, -4.0),
        (Kind::Labor, -3.0),
        (Kind::Consumer, -4.0),
        (Kind::Money, 12.0),
        (Kind::OrganicWaste, 2.0),
        (Kind::Culture, 2.0),
        ]);
    available.add_structure(
        "Brownstone Apt", vec![
        (Kind::Electricity, -1.0),
        (Kind::Water, -1.0),
        (Kind::FreshAir, -1.0),
        (Kind::Culture, -1.0),
        (Kind::Labor, 3.0),
        (Kind::Consumer, 2.0),
        (Kind::OrganicWaste, 2.0),
        (Kind::Greywater, 2.0),
        ]);
    available.add_structure(
        "Wine Bar", vec![
        (Kind::Wine, -7.0),
        (Kind::Labor, -3.0),
        (Kind::Consumer, -5.0),
        (Kind::Money, 14.0),
        (Kind::Leisure, 4.0),
        ]);
    available.add_structure(
        "Yoga Studio", vec![
        (Kind::Labor, -1.0),
        (Kind::Community, -4.0),
        (Kind::Leisure, -2.0),
        (Kind::Money, 6.0),
        (Kind::Fitness, 4.0),
        ]);
    available.add_structure(
        "Cross Support", vec![
        (Kind::Money, -1.2),
        ]);

    // Organic blocks

    available.add_structure(
        "Flower Garden", vec![
        (Kind::Groundwater, -1.0),
        (Kind::Fertilizer, -2.0),
        (Kind::Bees, 3.0),
        (Kind::Leisure, 2.0),
        (Kind::Community, 1.0),
        ]);
    available.add_structure(
        "Grass", vec![
        (Kind::Groundwater, -1.0),
        (Kind::Leisure, 3.0),
        ]);
    available.add_structure(
        "Corner Grass Hill", vec![
        (Kind::Groundwater, -1.0),
        (Kind::Leisure, 3.0),
        ]);
    available.add_structure(
        "Grass Hill", vec![
        (Kind::Groundwater, -1.0),
        (Kind::Leisure, 3.0),
        ]);
    available.add_structure(
        "Palm", vec![
        (Kind::Groundwater, -1.0),
        (Kind::FreshAir, 1.0),
        (Kind::Wilderness, 1.0),
        (Kind::Leisure, 3.0),
        ]);
    available.add_structure(
        "Oak Tree", vec![
        (Kind::Groundwater, -1.2),
        (Kind::FreshAir, 1.0),
        (Kind::Wilderness, 2.0),
        (Kind::Leisure, 1.0),
        (Kind::Acorns, 0.6),
        ]);
    available.add_structure(
        "Plum Tree", vec![
        (Kind::Groundwater, -1.0),
        (Kind::FreshAir, 2.0),
        (Kind::Wilderness, 1.0),
        (Kind::Leisure, 1.0),
        (Kind::Fruit, 0.25),
        ]);
    available.add_structure(
        "Pond", vec![
        (Kind::Groundwater, -1.0),
        (Kind::Water, 0.5),
        (Kind::Saltwater, 1.0),
        (Kind::Leisure, 1.0),
        ]);
    available.add_structure(
        "Sand", vec![
        (Kind::Leisure, 0.5),
        ]);
    available.add_structure(
        "Beech Tree Grove", vec![
        (Kind::Groundwater, -3.0),
        (Kind::FreshAir, 4.0),
        (Kind::Wilderness, 3.0),
        (Kind::Leisure, 1.0),
        ]);
    available.add_structure(
        "Beech Tree Grove", vec![
        (Kind::Groundwater, -3.0),
        (Kind::GreenhouseGas, -3.0),
        (Kind::FreshAir, 4.0),
        (Kind::Wilderness, 3.0),
        (Kind::Leisure, 1.0),
        ]);
    available.add_structure(
        "Elm Trees", vec![
        (Kind::Groundwater, -1.5),
        (Kind::FreshAir, 2.0),
        (Kind::Wilderness, 1.5),
        (Kind::Leisure, 1.0),
        ]);
    available.add_structure(
        "Ash Tree", vec![
        (Kind::Groundwater, -1.0),
        (Kind::FreshAir, 1.0),
        (Kind::Wilderness, 1.0),
        (Kind::Leisure, 2.0),
        ]);
    available.add_structure(
        "Maze Garden X", vec![
        (Kind::Groundwater, -1.0),
        (Kind::FreshAir, 1.0),
        (Kind::Leisure, 2.0),
        ]);
    available.add_structure(
        "Maze Garden I", vec![
        (Kind::Groundwater, -1.0),
        (Kind::FreshAir, 1.0),
        (Kind::Leisure, 2.0),
        ]);
    available.add_structure(
        "Maze Garden L", vec![
        (Kind::Groundwater, -1.0),
        (Kind::FreshAir, 1.0),
        (Kind::Leisure, 2.0),
        ]);
    available.add_structure(
        "Maze Garden O", vec![
        (Kind::Groundwater, -1.0),
        (Kind::FreshAir, 1.0),
        (Kind::Leisure, 2.0),
        ]);
    available.add_structure(
        "Maze Garden T", vec![
        (Kind::Groundwater, -1.0),
        (Kind::FreshAir, 1.0),
        (Kind::Leisure, 2.0),
        ]);
    available.add_structure(
        "Soil", vec![
        (Kind::Groundwater, 1.0),
        (Kind::Fertilizer, 1.0),
        ]);
    available.add_structure(
        "Soil", vec![
        (Kind::OrganicWaste, -1.0),
        (Kind::Groundwater, 1.0),
        (Kind::Fertilizer, 1.0),
        ]);
    available.add_structure(
        "Japanese Pine", vec![
        (Kind::Groundwater, -1.5),
        (Kind::FreshAir, 1.0),
        (Kind::Wilderness, 1.0),
        (Kind::Leisure, 2.0),
        ]);
    available.add_structure(
        "Japanese Pine", vec![
        (Kind::Groundwater, -1.5),
        (Kind::GreenhouseGas, -1.0),
        (Kind::FreshAir, 1.0),
        (Kind::Wilderness, 1.0),
        (Kind::Leisure, 2.0),
        ]);
    available.add_structure(
        "Poplar Tree", vec![
        (Kind::Groundwater, -3.0),
        (Kind::FreshAir, 4.0),
        (Kind::Wilderness, 2.0),
        (Kind::Leisure, 2.0),
        ]);
    available.add_structure(
        "Poplar Tree", vec![
        (Kind::Groundwater, -3.0),
        (Kind::GreenhouseGas, -2.0),
        (Kind::FreshAir, 4.0),
        (Kind::Wilderness, 2.0),
        (Kind::Leisure, 2.0),
        ]);
    available.add_structure(
        "Willow Tree", vec![
        (Kind::Groundwater, -3.0),
        (Kind::FreshAir, 2.0),
        (Kind::Wilderness, 2.0),
        (Kind::Leisure, 2.0),
        (Kind::Culture, 1.0),
        ]);
    available.add_structure(
        "Pine Tree", vec![
        (Kind::Groundwater, -1.0),
        (Kind::FreshAir, 2.0),
        (Kind::Wilderness, 1.0),
        (Kind::Leisure, 1.0),
        ]);
    available.add_structure(
        "Pine Tree", vec![
        (Kind::Groundwater, -1.0),
        (Kind::GreenhouseGas, -1.0),
        (Kind::FreshAir, 2.0),
        (Kind::Wilderness, 1.0),
        (Kind::Leisure, 1.0),
        ]);
    available.add_structure(
        "Apple Orchard", vec![
        (Kind::Groundwater, -2.0),
        (Kind::Labor, -1.0),
        (Kind::Fertilizer, -1.0),
        (Kind::Fruit, 4.0),
        (Kind::GreenhouseGas, 1.0),
        (Kind::OrganicWaste, 1.0),
        ]);
    available.add_structure(
        "Wetland", vec![
        (Kind::Water, -1.0),
        (Kind::Groundwater, 2.0),
        (Kind::Algae, 1.0),
        (Kind::Wilderness, 1.0),
        (Kind::Fertilizer, 1.0),
        ]);
    available.add_structure(
        "Wetland", vec![
        (Kind::Water, -1.0),
        (Kind::OrganicWaste, -2.0),
        (Kind::Groundwater, 2.0),
        (Kind::Algae, 1.0),
        (Kind::Wilderness, 1.0),
        (Kind::Fertilizer, 1.0),
        ]);
    available.add_structure(
        "Moss Campion", vec![
        (Kind::Groundwater, -0.2),
        (Kind::FreshAir, 0.25),
        (Kind::Wilderness, 0.25),
        ]);
    available.add_structure(
        "Bristlecone Pine", vec![
        (Kind::Groundwater, -1.0),
        (Kind::FreshAir, 2.0),
        (Kind::Wilderness, 1.0),
        ]);
    available.add_structure(
        "Olive Tree", vec![
        (Kind::Groundwater, -1.0),
        (Kind::FreshAir, 1.0),
        (Kind::Wilderness, 1.0),
        (Kind::Leisure, 2.0),
        (Kind::Olives, 0.5),
        ]);
    available.add_structure(
        "White Birch", vec![
        (Kind::Groundwater, -1.5),
        (Kind::FreshAir, 2.0),
        (Kind::Wilderness, 2.0),
        (Kind::Leisure, 2.0),
        ]);
    available.add_structure(
        "Mangrove", vec![
        (Kind::Saltwater, -1.5),
        (Kind::FreshAir, 1.0),
        (Kind::Wilderness, 1.0),
        (Kind::Water, 1.5),
        ]);
    available.add_structure(
        "Mangrove", vec![
        (Kind::Saltwater, -1.5),
        (Kind::Greywater, -1.0),
        (Kind::FreshAir, 1.0),
        (Kind::Wilderness, 1.0),
        (Kind::Water, 1.5),
        ]);
    available.add_structure(
        "Saguaro Cactus", vec![
        (Kind::Groundwater, -0.5),
        (Kind::FreshAir, 1.0),
        (Kind::Wilderness, 2.0),
        (Kind::Leisure, 2.0),
        ]);
    available.add_structure(
        "Soaptree Yucca", vec![
        (Kind::Groundwater, -0.5),
        (Kind::FreshAir, 0.5),
        (Kind::Wilderness, 1.0),
        (Kind::Leisure, 1.0),
        (Kind::Fruit, 0.5),
        ]);
    available.add_structure(
        "Barrel Cactus", vec![
        (Kind::Groundwater, -1.0),
        (Kind::FreshAir, 1.5),
        (Kind::Leisure, 1.0),
        ]);
    available.add_structure(
        "Foxtails", vec![
        (Kind::Groundwater, -1.0),
        (Kind::FreshAir, 0.5),
        (Kind::Wilderness, 2.0),
        (Kind::OrganicWaste, 0.5),
        ]);
    available.add_structure(
        "Wild Oats", vec![
        (Kind::Groundwater, -1.0),
        (Kind::FreshAir, 0.5),
        (Kind::Wilderness, 2.0),
        (Kind::Wheat, 0.5),
        ]);
    available.add_structure(
        "Bengal Bamboo", vec![
        (Kind::Groundwater, -1.0),
        (Kind::FreshAir, 1.0),
        (Kind::Wilderness, 2.0),
        (Kind::Vegetables, 0.2),
        ]);
    available.add_structure(
        "Kapok Tree", vec![
        (Kind::Groundwater, -2.0),
        (Kind::FreshAir, 1.0),
        (Kind::Wilderness, 3.0),
        (Kind::Leisure, 2.0),
        (Kind::RawCotton, 0.5),
        ]);
    available.add_structure(
        "Tualang Tree", vec![
        (Kind::Groundwater, -2.0),
        (Kind::FreshAir, 2.0),
        (Kind::Wilderness, 2.0),
        (Kind::Leisure, 3.0),
        ]);
    available.add_structure(
        "Umbrella Thorn Acacia", vec![
        (Kind::Groundwater, -0.6),
        (Kind::FreshAir, 1.0),
        (Kind::Wilderness, 1.5),
        (Kind::Leisure, 1.0),
        ]);
    available.add_structure(
        "Wild Date Palm", vec![
        (Kind::Groundwater, -2.5),
        (Kind::FreshAir, 1.0),
        (Kind::Wilderness, 2.0),
        (Kind::Fruit, 0.5),
        (Kind::Vegetables, 0.3),
        ]);
    available.add_structure(
        "White Spruce", vec![
        (Kind::Groundwater, -2.0),
        (Kind::FreshAir, 3.0),
        (Kind::Wilderness, 2.0),
        (Kind::Leisure, 2.0),
        ]);
    available.add_structure(
        "Arctic Moss", vec![
        (Kind::Groundwater, -0.2),
        (Kind::FreshAir, 0.5),
        (Kind::Wilderness, 0.5),
        (Kind::Algae, 0.5),
        ]);
    available.add_structure(
        "Arctic Willow", vec![
        (Kind::Groundwater, -0.4),
        (Kind::FreshAir, 0.6),
        (Kind::Wilderness, 1.0),
        ]);
    available.add_structure(
        "Canal L", vec![
        (Kind::Water, -1.0),
        (Kind::Groundwater, 1.0),
        (Kind::Greywater, 0.5),
        (Kind::Leisure, 1.0),
        (Kind::Algae, 0.1),
        ]);
    available.add_structure(
        "Canal +", vec![
        (Kind::Water, -1.0),
        (Kind::Groundwater, 1.0),
        (Kind::Greywater, 0.5),
        (Kind::Leisure, 1.0),
        ]);
    available.add_structure(
        "Canal I", vec![
        (Kind::Water, -1.0),
        (Kind::Groundwater, 1.0),
        (Kind::Greywater, 0.5),
        (Kind::Leisure, 1.0),
        (Kind::Fish, 0.1),
        ]);
    available.add_structure(
        "Canal T", vec![
        (Kind::Water, -1.0),
        (Kind::Groundwater, 1.0),
        (Kind::Greywater, 0.5),
        (Kind::Leisure, 1.0),
        ]);
    available.add_structure(
        "Shaded Coffee Field", vec![
        (Kind::Groundwater, -1.5),
        (Kind::Labor, -2.0),
        (Kind::Fertilizer, -2.0),
        (Kind::CoffeeCherry, 3.0),
        (Kind::Fruit, 0.2),
        (Kind::OrganicWaste, 1.0),
        ]);
    available.add_structure(
        "Sun Coffee Field", vec![
        (Kind::Groundwater, -2.0),
        (Kind::Labor, -1.0),
        (Kind::Fertilizer, -2.0),
        (Kind::Pesticides, -1.0),
        (Kind::CoffeeCherry, 6.0),
        (Kind::GreenhouseGas, 0.2),
        (Kind::OrganicWaste, 1.0),
        ]);
    println!("Available structures:");
    available.print();
    let mut current = &available;
    let mut search = Cluster { name: "asdf".to_string(), structures: Vec::new() };
    loop {
        println!("Selected structures:");
        selected.print();

        println!("Number to add structure, -Number to remove, text to search");
        let s = &mut String::new();
        // TODO: trim earlier
        io::stdin().read_line(s).unwrap();
        match s.trim().parse::<isize>() { 
            Ok(i) if i > 0=> {
                let i = (i - 1) as usize;
                if current.structures.len() > i {
                    selected.structures.push(current.structures[i].clone());
                }
            },
            Ok(i) if i < 0 => {
                let i = (i * -1 - 1) as usize;
                if selected.structures.len() > i {
                    selected.structures.remove(i);
                }
            },
            Ok(_) => { },
            Err(_) => {
                if s.trim() == "" {
                    current = &available;
                } else {
                    search = available.search(s.trim());
                    current = &search;
                }
                println!("Search structures:");
                current.print();
            }
        };

    }
}
