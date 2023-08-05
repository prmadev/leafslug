//! different restrictions

use super::ingrediants::Ingredient;

/// restrictiions that come with a lifestyle choice
pub enum Restriction {
    Ingredients(Vec<Ingredient>),
    Preparation(String),
}
