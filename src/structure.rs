use crate::kind::Kind;
use crate::resource::Resource;

#[derive(Clone)]
pub struct Structure {
    pub name: String,
    pub resources: Vec<Resource>,
}

impl Structure {
    pub fn new(name: &str, resources: Vec<(Kind, f64)>) -> Structure {
        Structure {
            name: name.to_string(),
            resources: resources.iter().map(
                |r| Resource::new(r.0.clone(), r.1)
                ).collect(),
        }
    }
    pub fn amount_for(&self, kind: Kind) -> f64 {
        self.resources.iter().find(|r| r.kind == kind).map(|r| r.amount).unwrap_or(0.0)
    }

    pub fn searchable_content(&self) -> String {
        format!("{} {}", self.name, self.resource_names().join(" ")).to_lowercase()
    }
    pub fn resource_names(&self) -> Vec<String> {
        self.resources.iter().map(|r| r.kind.name() ).collect()
    }
}
