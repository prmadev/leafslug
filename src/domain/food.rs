//! food is a food

use super::{lifestyle::Lifestyle, restrictions::Restriction};

pub struct Food {
    pub id: i128,
    pub name: String,
    pub incompatibilities: Vec<Incompatibility>,
    pub substitutes: Vec<Food>,
}

pub struct Incompatibility {
    pub with: Lifestyle,
    pub restrictions: Restriction,
    pub reason: String,
}
