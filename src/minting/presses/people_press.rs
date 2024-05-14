use std::{sync::Arc, io::Error};

use crate::{unit_pools::UnitArchipelago, minting::presses::common::people::full_name_die};

use super::common::people::{skill_die, trait_die};

pub fn people_press(data: Arc<UnitArchipelago>) -> Result<String, Error> {
    let full_name = full_name_die(data.clone())?;
    let skill0 = skill_die(data.clone())?;
    let skill1 = skill_die(data.clone())?;
    let skill2 = skill_die(data.clone())?;
    let trait0 = trait_die(data.clone())?;
    let trait1 = trait_die(data.clone())?;
    let trait2 = trait_die(data)?;
    Ok(format!("{} They are a {}, a {}, and a {}. They are {}, {} and {}.", full_name, skill0, skill1, skill2, trait0, trait1, trait2))
}
