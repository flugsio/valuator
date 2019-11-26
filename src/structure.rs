use crate::kind::Kind;

#[derive(Clone)]
pub struct Structure {
    pub name: String,
    pub resources: Vec<super::resource::Resource>,
}

impl Structure {
    pub fn amount_for(&self, kind: Kind) -> i64 {
        self.resources.iter().find(|r| r.kind == kind).map(|r| r.amount).unwrap_or(0)
    }
}
