use crate::minting::presses::{people_press::PeopleObject, place_press::PlaceObject};

#[derive(Debug, Clone, Copy)]
pub enum MintingType {
    Place,
    People,
    Artifact,
    Operation,
    ShipName,
    ShipClass,
    Currency,
    MetalAndAlloy,
    Empire,
    Government,
    Language,
}

pub type SkillAmount = u8;
#[derive(Debug, Clone, Copy)]
pub enum SkillLevel {
    Low,
    Medium,
    High,
    Ultra,
    Any,
}
pub type TraitAmount = u8;

#[derive(Debug, Clone)]
pub enum MintingObjectType {
    PlaceObject,
    /// To Create a Person Object.
    ///
    /// `SkillAmount` is the amount of skills the person has.
    /// `SkillLevel` is the level of the skill.
    ///     - `Any` means any skill level.
    ///     - `Low` means low skill level. (e.g. novice)
    ///     - `Medium` means medium skill level. (e.g. professional)
    ///     - `High` means high skill level. (e.g. master)
    ///     - `Ultra` means ultra skill level. (e.g. legend)
    /// `TraitAmount` is the amount of traits the person has.
    PeopleObject(Vec<(SkillAmount, SkillLevel)>, TraitAmount),
    OperationObject,
    MetalAndAlloyObject,
}

#[derive(Debug, Clone)]
pub enum ObjectResult {
    PlaceObject(PlaceObject),
    PeopleObject(PeopleObject),
    OperationObject(String),
    MetalAndAlloyObject(String),
}
