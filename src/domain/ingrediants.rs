//! ingredients are the ingrediates

/// things that make up other things
pub struct Ingredient {
    pub id: i128,
    pub name: String,
    pub sustitutes: Vec<Ingredient>,
}
