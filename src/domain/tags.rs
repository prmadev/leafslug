//! tags of the different items

/// Tags for each item
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq, Debug)]
pub struct Tag {
    /// Id is the machin friendly identification
    pub id: Option<i32>,
    /// Names may overlap, but we should avoid that as much as possible.
    pub name: String,
    /// Alias for the name of the item
    pub aliases: Vec<String>,
}
