pub mod minting_result;
pub mod minting_type;
mod presses;

pub mod minters {
    use std::io::Error;

    use fortuna::{Fortuna, RngApi};
    use thoth::Thoth;

    use crate::{
        minting::{
            minting_type::{ObjectResult, SkillAmount, SkillLevel, TraitAmount},
            presses::{
                metal_alloy_press::metal_alloy_object_press,
                operation_press::operation_object_press, people_press::people_object_press,
                place_press::place_object_press,
            },
        },
        unit_pools::UnitArchipelago,
    };

    use super::presses::{
        artifact_press::artifact_press, currency_press::currency_press, empire_press::empire_press,
        government_press::government_press, language_press::language_press,
        metal_alloy_press::metal_alloy_press, operation_press::operation_press,
        people_press::people_press, place_press::place_press, ship_class_press::ship_class_press,
        ship_name_press::ship_name_press,
    };

    pub fn mint_place(data: &UnitArchipelago, rng: &mut Fortuna) -> Result<String, Error> {
        place_press(data, rng)
    }

    pub fn mint_place_object(
        data: &UnitArchipelago,
        rng: &mut Fortuna,
    ) -> Result<ObjectResult, Error> {
        place_object_press(data, rng)
    }

    pub fn mint_people(data: &UnitArchipelago, rng: &mut Fortuna) -> Result<String, Error> {
        people_press(data, rng)
    }
    pub fn mint_people_object(
        data: &UnitArchipelago,
        rng: &mut Fortuna,
        skill_amount: SkillAmount,
        skill_level: SkillLevel,
        trait_amount: TraitAmount,
    ) -> Result<ObjectResult, Error> {
        people_object_press(data, rng, skill_amount, skill_level, trait_amount)
    }

    pub fn mint_language(data: &UnitArchipelago, rng: &mut Fortuna) -> Result<String, Error> {
        language_press(data, rng)
    }

    pub fn mint_metal_alloy(data: &UnitArchipelago, rng: &mut Fortuna) -> Result<String, Error> {
        metal_alloy_press(data, rng)
    }

    pub fn mint_metal_alloy_object(
        data: &UnitArchipelago,
        rng: &mut Fortuna,
    ) -> Result<ObjectResult, Error> {
        metal_alloy_object_press(data, rng)
    }

    pub fn mint_artifact(data: &UnitArchipelago, rng: &mut Fortuna) -> Result<String, Error> {
        artifact_press(data, rng)
    }

    pub fn mint_operation(data: &UnitArchipelago, rng: &mut Fortuna) -> Result<String, Error> {
        operation_press(data, rng)
    }

    pub fn mint_operation_object(
        data: &UnitArchipelago,
        rng: &mut Fortuna,
    ) -> Result<ObjectResult, Error> {
        operation_object_press(data, rng)
    }

    pub fn mint_government(data: &UnitArchipelago, rng: &mut Fortuna) -> Result<String, Error> {
        government_press(data, rng, "")
    }

    pub fn mint_empire(data: &UnitArchipelago, rng: &mut Fortuna) -> Result<String, Error> {
        empire_press(data, rng)
    }

    pub fn mint_ship_name(data: &UnitArchipelago, rng: &mut Fortuna) -> Result<String, Error> {
        ship_name_press(data, rng, "")
    }

    pub fn mint_ship_class(data: &UnitArchipelago, rng: &mut Fortuna) -> Result<String, Error> {
        let (avg_speed, avg_range) = {
            if let Ok(speed) = rng.random_from_f32_range(0.5, 5.5)
                && let Ok(range) = rng.random_from_f32_range(100.5, 1000.5)
            {
                (speed, range)
            } else {
                return Err(Error::other("Unable to generate ship class".to_string()));
            }
        };
        ship_class_press(data, rng, "", avg_speed, avg_range)
    }

    pub fn mint_currency(
        data: &UnitArchipelago,
        rng: &mut Fortuna,
        unicode: &Thoth,
    ) -> Result<String, Error> {
        currency_press(data, rng, unicode, "")
    }
}
