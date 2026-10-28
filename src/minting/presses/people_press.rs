use std::{
    collections::{HashMap, HashSet},
    io::Error,
};

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

/// Create a new person.
///
/// # Arguments
///
/// * `data` - Unit archipelago
/// * `rng` - Random number generator
/// * `skills` - Vec of (skill amount, skill level)
/// * `trait_amount` - Amount of traits
///
/// # Notes
/// Skills and traits are unique.
///
/// The amount of skills and traits may be, seldom, lower than the amount specified.
pub fn people_object_press(
    data: &UnitArchipelago,
    rng: &mut Fortuna,
    skills: Vec<(SkillAmount, SkillLevel)>,
    trait_amount: TraitAmount,
) -> Result<ObjectResult, Error> {
    let first_name = first_name_type(data, rng)?;
    let last_name = last_name_type(data, rng)?;
    let legal_name = format!("{} {}", first_name, last_name);
    let skills = {
        let mut tmp_map: HashMap<String, String> = HashMap::new();
        for (skill_amount, skill_level) in skills {
            for _ in 0..skill_amount {
                // NOTE: Not that happy with this loop structure
                // But it works and is simple
                let mut tries = 0;

                loop {
                    let (level, skill) = level_capped_skill_die(data, rng, skill_level)?;
                    if !tmp_map.contains_key(&skill) {
                        tmp_map.insert(skill, level);
                        break;
                    } else if tries > 10 {
                        break;
                    }
                    tries += 1;
                }
            }
        }
        let mut tmp: Vec<(String, String)> = Vec::new();
        for (skill, level) in tmp_map {
            tmp.push((skill, level));
        }
        tmp
    };
    let traits = {
        let mut tmp_map = HashSet::new();
        for _ in 0..trait_amount {
            // NOTE: Again, not that happy with this loop structure
            // But it works and is simple
            let mut tries = 0;

            loop {
                let trait_name = trait_die(data, rng)?;
                if !tmp_map.contains(&trait_name) {
                    tmp_map.insert(trait_name);
                    break;
                } else if tries > 10 {
                    break;
                }
                tries += 1;
            }
        }
        let mut tmp: Vec<String> = Vec::new();
        for trait_name in tmp_map {
            tmp.push(trait_name.to_string());
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
        if let Ok(n) = rng.random_from_range(0, 2) {
            if n == 0
                && let Ok(idx) =
                    rng.random_index(data.lexical_unit_lagoon.people_trait_ck3.unit_pool.len())
            {
                Ok(&data.lexical_unit_lagoon.people_trait_ck3.unit_pool[idx])
            } else if n == 1
                && let Ok(idx) = rng.random_index(
                    data.lexical_unit_lagoon
                        .people_trait_rolls_ck2_mod
                        .unit_pool
                        .len(),
                )
            {
                Ok(&data
                    .lexical_unit_lagoon
                    .people_trait_rolls_ck2_mod
                    .unit_pool[idx])
            } else if let Ok(idx) =
                rng.random_index(data.lexical_unit_lagoon.people_trait_ck2.unit_pool.len())
            {
                Ok(&data.lexical_unit_lagoon.people_trait_ck2.unit_pool[idx])
            } else {
                Err(Error::other("trait_die failed"))
            }
        } else {
            Err(Error::other("trait_die failed"))
        }
    }
}
