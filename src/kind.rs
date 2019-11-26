#[derive(Clone, PartialEq, Eq, Ord, PartialOrd)]
pub enum Kind {
    Electricity,
    GroundWater,
    Money,
    Water,
    Fertilizer,
    OrganicWaste,
}

impl Kind {
    pub fn name(&self) -> String {
        match *self {
            Kind::Electricity => "Electricity",
            Kind::GroundWater => "GroundWater",
            Kind::Money => "Money",
            Kind::Water => "Water",
            Kind::Fertilizer => "Fertilizer",
            Kind::OrganicWaste => "OrganicWaste",
        }.to_string()
    }
}
