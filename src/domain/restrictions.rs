//! different restrictions

use super::ingrediants::Ingredient;

/// restrictiions that come with a lifestyle choice
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq, Debug)]
pub enum Restriction {
    /// This means that the restriction is about the ingredients of the food.
    /// For example, eggs, meat and dairy are the ingredients that are restricted.
    Ingredients(Vec<Ingredient>),
    /// This specifies the restrictions about making the food.
    /// for example, if the making of x is done without paying the workers well.
    Preparation(String),
}
