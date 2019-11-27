use crate::kind::Kind;

// Resource, incoming/outgoing from the structure
#[derive(Clone)]
pub struct Resource {
    pub kind: Kind,
    pub amount: i64,
}

impl Resource {
    pub fn new(kind: Kind, amount: i64) -> Resource {
        Resource {
            kind: kind,
            amount: amount,
        }
    }
}
