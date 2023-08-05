//! lifestyle is the life style choice of the user

use super::restrictions::Restriction;

pub struct Lifestyle {
    pub id: i128,
    pub name: String,
    pub restrictions: Vec<Restriction>,
    pub compatible_with: Vec<Lifestyle>,
}
