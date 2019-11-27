use std::collections::BTreeSet;
use crate::kind::Kind;
use crate::structure::Structure;

pub struct Cluster {
    pub name: String,
    pub structures: Vec<Structure>,
}

impl Cluster {
    pub fn print(&self) {
        let kinds = Cluster::unique_kinds(&self);

        print!("{:>15} |", "");
        for kind in kinds.clone() {
            print!("{:>12} |", kind.name());
        };
        println!("");

        for structure in &self.structures {
            print!("{:>15} |", structure.name);
            for kind in kinds.clone() {
                print!("{:>12} |", structure.amount_for(kind));
            };
            println!("");
        };
    }

    pub fn add_structure(&mut self, name: &str, resources: Vec<(Kind, i64)>) {
        self.structures.push(Structure::new(name, resources));
    }

    pub fn unique_kinds(&self) -> BTreeSet<Kind> {
        self.structures.clone().into_iter().flat_map(|s| s.resources).map(|r| r.kind).collect()
    }
}
