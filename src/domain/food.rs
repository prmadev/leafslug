//! Food is a product which is the subject of strict questioning

use crate::Tag;

/// Product is the holder of an instance of "product"
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq, Debug)]
pub struct Product {
    /// Id is the machin friendly identification
    pub id: Option<i32>,
    /// Names may overlap, but we should avoid that as much as possible.
    pub name: String,
    /// aliases
    pub aliases: Vec<String>,
    /// This is the list of incompatibilies with different lifestyles
    pub tags: Vec<Tag>,
    /// If there are extremely similar foods which form a different form of incampatibility
    pub substitutes: Vec<Product>,
}
