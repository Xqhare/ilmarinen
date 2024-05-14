pub mod people {
    use std::{sync::Arc, io::Error};
    
    use tyche::prelude;

    use crate::unit_pools::UnitArchipelago;

    use super::common::comp_name_die;

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

mod common {
    use std::{sync::Arc, io::Error};

    use tyche::prelude;

    use crate::unit_pools::UnitArchipelago;

    pub fn comp_name_die(data: Arc<UnitArchipelago>) -> Result<String, Error> {
        Ok(format!("{}{}", data.lexical_unit_lagoon.general_comp0.unit_pool[prelude::random_index(data.lexical_unit_lagoon.general_comp0.unit_pool.len())?], data.lexical_unit_lagoon.general_comp1.unit_pool[prelude::random_index(data.lexical_unit_lagoon.general_comp1.unit_pool.len())?]))
    }
}


