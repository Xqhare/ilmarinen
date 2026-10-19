use std::io::Error;

use fortuna::Fortuna;

use crate::{
    minting::{
        minting_type::{ObjectResult, SkillAmount, SkillLevel, TraitAmount},
        presses::common::{first_name_type, last_name_type, level_capped_skill_die, nickname_type},
    },
    unit_pools::UnitArchipelago,
};

use self::people::trait_die;

use super::common::{full_people_name_die, skill_die};

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct PeopleObject {
    pub legal_name: String,
    pub nick_name: String,
    pub first_name: String,
    pub last_name: String,
    /// Vec of (skill name, skill level)
    pub skills: Vec<(String, String)>,
    pub traits: Vec<String>,
}

pub fn people_object_press(
    data: &UnitArchipelago,
    rng: &mut Fortuna,
    skill_amount: SkillAmount,
    skill_level: SkillLevel,
    trait_amount: TraitAmount,
) -> Result<ObjectResult, Error> {
    let first_name = first_name_type(data, rng)?;
    let last_name = last_name_type(data, rng)?;
    let legal_name = format!("{} {}", first_name, last_name);
    let skills = {
        let mut tmp = Vec::with_capacity(skill_amount as usize);
        for _ in 0..skill_amount {
            tmp.push(level_capped_skill_die(data, rng, skill_level)?);
        }
        tmp
    };
    let traits = {
        let mut tmp = Vec::with_capacity(trait_amount as usize);
        for _ in 0..trait_amount {
            tmp.push(trait_die(data, rng)?.to_string());
        }
        tmp
    };
    Ok(ObjectResult::PeopleObject(PeopleObject {
        legal_name,
        nick_name: nickname_type(data, rng)?,
        first_name,
        last_name,
        skills,
        traits,
    }))
}

pub fn people_press(data: &UnitArchipelago, rng: &mut Fortuna) -> Result<String, Error> {
    Ok(format!(
        "{} They are a {}, a {}, and a {}. They are {}, {} and {}.",
        full_people_name_die(data, rng)?,
        skill_die(data, rng)?,
        skill_die(data, rng)?,
        skill_die(data, rng)?,
        trait_die(data, rng)?,
        trait_die(data, rng)?,
        trait_die(data, rng)?
    ))
}

mod people {
    use std::io::Error;

    use fortuna::{Fortuna, RngApi};

    use crate::unit_pools::UnitArchipelago;

    pub fn trait_die<'a>(data: &'a UnitArchipelago, rng: &mut Fortuna) -> Result<&'a str, Error> {
        if let Ok(n) = rng.random_from_range(0, 1)
            && n == 0
            && let Ok(idx) =
                rng.random_index(data.lexical_unit_lagoon.people_trait_ck3.unit_pool.len())
        {
            Ok(&data.lexical_unit_lagoon.people_trait_ck3.unit_pool[idx])
        } else if let Ok(idx) =
            rng.random_index(data.lexical_unit_lagoon.people_trait_ck2.unit_pool.len())
        {
            Ok(&data.lexical_unit_lagoon.people_trait_ck2.unit_pool[idx])
        } else {
            Err(Error::other("trait_die failed"))
        }
    }
}
