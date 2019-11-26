
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
    available.structures.push(Structure { name: "Sprinkler".to_string(),
        resources: vec![
            Resource::new(Kind::Electricity, -2),
            Resource::new(Kind::Water, -1),
            Resource::new(Kind::GroundWater, 1),
        ],
    });
    available.structures.push(Structure { name: "Water Tower".to_string(),
        resources: vec![
            Resource::new(Kind::Money, -2),
            Resource::new(Kind::Water, 8),
        ],
    });
    available.structures.push(Structure { name: "Soil".to_string(),
        resources: vec![
            Resource::new_optional(Kind::OrganicWaste, -1),
            Resource::new(Kind::GroundWater, 1),
            Resource::new(Kind::Fertilizer, 1),
        ],
    });
    println!("Available structures:");
    available.print();
}
