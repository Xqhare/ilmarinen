use std::{sync::Arc, io::Error};

use crate::unit_pools::UnitArchipelago;

use self::people::trait_die;

use super::common::{skill_die, full_people_name_die};

pub fn people_press(data: Arc<UnitArchipelago>) -> Result<String, Error> {
    Ok(
        format!("{} They are a {}, a {}, and a {}. They are {}, {} and {}.",
            full_people_name_die(data.clone())?,
            skill_die(data.clone())?,
            skill_die(data.clone())?,
            skill_die(data.clone())?,
            trait_die(data.clone())?,
            trait_die(data.clone())?,
            trait_die(data)?
        )
    )
}

mod people {
    use std::{sync::Arc, io::Error};
    
    use tyche::prelude;

    use crate::unit_pools::UnitArchipelago;    

    pub fn trait_die(data: Arc<UnitArchipelago>) -> Result<String, Error> {
        if prelude::random_from_range(0, 1)? == 0 {
            Ok(
                data.lexical_unit_lagoon.people_trait_ck3.unit_pool[prelude::random_index(data.lexical_unit_lagoon.people_trait_ck3.unit_pool.len())?].clone()
            )
        } else {
            Ok(
                data.lexical_unit_lagoon.people_trait_ck2.unit_pool[prelude::random_index(data.lexical_unit_lagoon.people_trait_ck2.unit_pool.len())?].clone()
            )
        }
    }
}
