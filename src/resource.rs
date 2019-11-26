
// Resource, incoming/outgoing from the structure
#[derive(Clone)]
pub struct Resource {
    pub kind: super::kind::Kind,
    pub amount: i64,
    pub optional: bool,
}

impl Resource {
    pub fn new(kind: super::kind::Kind, amount: i64) -> Resource {
        Resource {
            kind: kind,
            amount: amount,
            optional: false,
        }
    }

    pub fn new_optional(kind: super::kind::Kind, amount: i64) -> Resource {
        Resource {
            kind: kind,
            amount: amount,
            optional: true,
        }
    }
}
