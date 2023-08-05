//! ingredients are the ingrediates

/// things that make up other things
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq, Debug)]
pub struct Ingredient {
    pub id: i128,
    pub name: String,
    pub sustitutes: Vec<Ingredient>,
}
