use std::{sync::Arc, io::Error};

use crate::unit_pools::UnitArchipelago;

use self::people::{full_name_die, skill_die, trait_die};

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

pub mod people {
    use std::{sync::Arc, io::Error};
    
    use tyche::prelude;

    use crate::{unit_pools::UnitArchipelago, minting::presses::common::common::comp_name_die};

    pub fn full_name_die(data: Arc<UnitArchipelago>) -> Result<String, Error> {
        Ok(format!("Name: {}. Nickname: {}", legal_name_die(data.clone())?, nickname_type(data)?))
    }

    pub fn legal_name_die(data: Arc<UnitArchipelago>) -> Result<String, Error> {
        Ok(format!("{} {}", first_name_type(data.clone())?, last_name_type(data)?))
    }

    fn nickname_type(data: Arc<UnitArchipelago>) -> Result<String, Error> {
        Ok(format!("{}", if prelude::random_from_range(0, 2)? == 0 {
                data.lexical_unit_lagoon.people_nickname.unit_pool[prelude::random_index(data.lexical_unit_lagoon.people_nickname.unit_pool.len())?].clone()
            } else {
                String::default()
            }))
    }

    fn first_name_type(data: Arc<UnitArchipelago>) -> Result<String, Error> {
        if prelude::random_from_range(0, 1)? == 0 {
            comp_name_die(data)
        } else {
            Ok(data.lexical_unit_lagoon.people_first_name.unit_pool[prelude::random_index(data.lexical_unit_lagoon.people_first_name.unit_pool.len())?].clone())
        }
    }
    
    fn last_name_type(data: Arc<UnitArchipelago>) -> Result<String, Error> {
        if prelude::random_from_range(0, 1)? == 0 {
            comp_name_die(data)
        } else {
            Ok(data.lexical_unit_lagoon.people_last_name.unit_pool[prelude::random_index(data.lexical_unit_lagoon.people_last_name.unit_pool.len())?].clone())
        }
    }

    pub fn skill_die(data: Arc<UnitArchipelago>) -> Result<String, Error> {
        Ok(format!("{} {}", data.lexical_unit_lagoon.people_skill_level.unit_pool[prelude::random_index(data.lexical_unit_lagoon.people_skill_level.unit_pool.len())?], data.lexical_unit_lagoon.people_skill.unit_pool[prelude::random_index(data.lexical_unit_lagoon.people_skill.unit_pool.len())?]))
    }

    pub fn trait_die(data: Arc<UnitArchipelago>) -> Result<String, Error> {
        if prelude::random_from_range(0, 1)? == 0 {
            Ok(data.lexical_unit_lagoon.people_trait_ck3.unit_pool[prelude::random_index(data.lexical_unit_lagoon.people_trait_ck3.unit_pool.len())?].clone())
        } else {
            Ok(data.lexical_unit_lagoon.people_trait_ck2.unit_pool[prelude::random_index(data.lexical_unit_lagoon.people_trait_ck2.unit_pool.len())?].clone())
        }
    }
}
