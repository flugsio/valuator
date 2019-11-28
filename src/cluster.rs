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

        print!("    {:>20} |", "");
        for kind in kinds.clone() {
            print!(" {} |", kind.name());
        };
        println!("");

        for (i, structure) in (&self.structures).iter().enumerate() {
            print!("{:>3} {:>20} |", i + 1, structure.name);
            for kind in kinds.clone() {
                print!("{:>width$} |",
                       structure.amount_for(kind.clone()),
                       width = kind.name().len() + 1);
            };
            println!("");
        };
    }

    // returns a new cluster set of structures by filtering on search term
    pub fn search(&self, search: &str) -> Cluster {
        let mut structures = Vec::new();
        for structure in &self.structures {
            if structure.searchable_content().contains(&search.to_lowercase()) {
                structures.push(structure.clone());
            }
        }
        Cluster {
            name: search.to_string(),
            structures: structures,
        }
    }

    pub fn add_structure(&mut self, name: &str, resources: Vec<(Kind, f64)>) {
        self.structures.push(Structure::new(name, resources));
    }

    pub fn unique_kinds(&self) -> BTreeSet<Kind> {
        self.structures.clone().into_iter().flat_map(|s| s.resources).map(|r| r.kind).collect()
    }
}
