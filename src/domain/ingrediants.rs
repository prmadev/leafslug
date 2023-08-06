//! ingredients are the ingrediates

/// things that make up other things
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq, Debug)]
pub struct Ingredient {
    /// Id is the machin friendly identification of type
    pub id: i128,
    /// Names may overlap, but we should avoid that as much as possible.
    pub name: String,
    /// If there are extremely similar ingredients which are used for a different form of incampatibility
    pub sustitutes: Vec<Ingredient>,
}
