//! lifestyle is the life style choice of the user

use super::restrictions::Restriction;

#[derive(serde::Deserialize, serde::Serialize, Clone, PartialEq, Eq, Debug)]
/// the lifestyle choice of person that comes with a set of restrictions
///
pub struct Lifestyle {
    /// Id is the machin friendly identification of type
    pub id: Option<i32>,
    /// Names may overlap, but we should avoid that as much as possible.
    pub name: String,
    /// list of restrictions that are carried out by this lifestyle choice
    pub restrictions: Vec<Restriction>,
    /// list of lifestyles that are fully compatible with this life style.
    /// So, for example, if this instance is an instance of vegeterianism,
    /// veganism is compatible with it. so anything that is vegan, can also
    /// be counted as vegetarian as well, but not the other way around.
    pub compatible_with: Vec<Lifestyle>,
}
