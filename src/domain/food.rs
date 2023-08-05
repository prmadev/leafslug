//! food is a food

use super::{lifestyle::Lifestyle, restrictions::Restriction};

#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq, Debug)]
pub struct Food {
    pub id: i128,
    pub name: String,
    pub incompatibilities: Vec<Incompatibility>,
    pub substitutes: Vec<Food>,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq, Debug)]
pub struct Incompatibility {
    pub with: Lifestyle,
    pub restrictions: Restriction,
    pub reason: String,
}
