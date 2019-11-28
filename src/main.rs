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
        "Sprinklers", vec![
        (Kind::Electricity, -0.5),
        (Kind::Water, -1.0),
        (Kind::Groundwater, 1.0),
        ]);
    available.add_structure(
        "Water Tower", vec![
        (Kind::Money, -2.0),
        (Kind::Water, 8.0),
        ]);
    available.add_structure(
        "Soil", vec![
        (Kind::Groundwater, 1.0),
        (Kind::Fertilizer, 1.0),
        ]);
    available.add_structure(
        "Soil+", vec![
        (Kind::OrganicWaste, -1.0),
        (Kind::Groundwater, 1.0),
        (Kind::Fertilizer, 1.0),
        ]);
    available.add_structure(
        "Algae Farm", vec![
        (Kind::Greywater, -8.0),
        (Kind::Algae, 4.0),
        (Kind::Water, 8.0),
        ]);
    available.add_structure(
        "Algae Farm+", vec![
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
        "Corn Field", vec![
        (Kind::Groundwater, -1.0),
        (Kind::Labor, -1.0),
        (Kind::Fertilizer, -1.0),
        (Kind::Corn, 3.0),
        (Kind::GreenhouseGas, 1.0),
        ]);
    available.add_structure(
        "Grain Silo", vec![
        (Kind::Corn, -4.0),
        (Kind::Corn, 2.0),
        (Kind::Money, 8.0),
        ]);
    available.add_structure(
        "Ethanol Distillery", vec![
        (Kind::Corn, -6.0),
        (Kind::Water, -4.0),
        (Kind::Electricity, 2.0),
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
        "Community Center", vec![
        (Kind::Electricity, -1.0),
        (Kind::Money, -1.0),
        (Kind::Community, 5.0),
        (Kind::SortedWaste, 2.0),
        ]);
    available.add_structure(
        "Community Center+", vec![
        (Kind::Electricity, -1.0),
        (Kind::Money, -1.0),
        (Kind::InorganicWaste, -2.0),
        (Kind::Community, 5.0),
        (Kind::SortedWaste, 2.0),
        ]);
    available.add_structure(
        "Plaza", vec![
        (Kind::Money, -1.0),
        (Kind::Community, 1.0),
        (Kind::Leisure, 2.0),
        ]);
    available.add_structure(
        "Solar Tree", vec![
        (Kind::Money, -2.0),
        (Kind::Electricity, 1.0),
        (Kind::Community, 1.0),
        (Kind::Leisure, 1.0),
        ]);
    available.add_structure(
        "Beech Tree Grove", vec![
        (Kind::Groundwater, -3.0),
        (Kind::FreshAir, 4.0),
        (Kind::Wilderness, 3.0),
        (Kind::Leisure, 1.0),
        ]);
    available.add_structure(
        "Beech Tree Grove+", vec![
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
        "Shack I/II/III", vec![
        (Kind::Water, -1.0),
        (Kind::Community, -1.0),
        (Kind::Labor, 5.0),
        (Kind::Risk, 2.0),
        (Kind::OrganicWaste, 4.0),
        (Kind::Greywater, 4.0),
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
