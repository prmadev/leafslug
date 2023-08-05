//! different restrictions

use super::ingrediants::Ingredient;

/// restrictiions that come with a lifestyle choice
#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq, Debug)]
pub enum Restriction {
    Ingredients(Vec<Ingredient>),
    Preparation(String),
}
