
mod cluster;
mod structure;
mod resource;
mod kind;

use cluster::Cluster;
use structure::Structure;
use resource::Resource;
use kind::Kind;

fn main() {
    let mut available = Cluster {
        name: "Available".to_string(),
        structures: Vec::new(),
    };
    available.add_structure(
        "Sprinklers", vec![
        (Kind::Electricity, -2),
        (Kind::Water, -1),
        (Kind::GroundWater, 1),
        ]);
    available.add_structure(
        "Water Tower", vec![
        (Kind::Money, -2),
        (Kind::Water, 8),
        ]);
    available.add_structure(
        "Soil", vec![
        (Kind::GroundWater, 1),
        (Kind::Fertilizer, 1),
        ]);
    available.add_structure(
        "Soil+", vec![
        (Kind::OrganicWaste, -1),
        (Kind::GroundWater, 1),
        (Kind::Fertilizer, 1),
        ]);
    // Algae Farm
    // Anaerobic Digestor
    // Water Tower
    // Water Well
    // Corn Field
    // Grain Silo
    // Ethanol Distillery
    // Ethanol Generator
    // Community Center
    // Plaza
    // Solar Tree
    // Beech Tree Grove
    // Elm Trees
    // Ash Tree
    // Shack I
    println!("Available structures:");
    available.print();
}
