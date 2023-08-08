//! Food is a product which is the subject of strict questioning

use super::{lifestyle::Lifestyle, restrictions::Restriction};

/// Food is the holder of an instance of "food"
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq, Debug)]
pub struct Food {
    /// Id is the machin friendly identification of type food
    pub id: Option<i32>,
    /// Names may overlap, but we should avoid that as much as possible.
    pub name: String,
    /// This is the list of incompatibilies with different lifestyles
    pub incompatibilities: Vec<Incompatibility>,
    /// If there are extremely similar foods which form a different form of incampatibility
    pub substitutes: Vec<Food>,
}

/// specifies an specific incompatbility
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq, Debug)]
pub struct Incompatibility {
    /// specifies the life style that it is incompatible to  
    pub with: Lifestyle,
    ///  specifies the restriction that it violates
    pub restrictions: Restriction,
    ///  description of how this restriction is violated by the food
    pub reason: String,
}
