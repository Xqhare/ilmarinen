use std::{sync::Arc, io::Error};

use crate::unit_pools::UnitArchipelago;

use self::place::{adj_die, place_description_die};

use super::common::full_place_name_die;


pub fn place_press(data: Arc<UnitArchipelago>) -> Result<String, Error> {
    Ok(
        format!("{}{}. {}",
            adj_die(data.clone())?,
            full_place_name_die(data.clone())?,
            place_description_die(data)?
        )
    )
}

mod place {
    use std::{io::Error, sync::Arc, ops::Add};

    use tyche::prelude;

    use crate::{unit_pools::UnitArchipelago, minting::presses::common::simple_name_type};

    pub fn adj_die(data: Arc<UnitArchipelago>) -> Result<String, Error> {
        if prelude::random_from_range(0, 1)? == 0 {
            Ok(simple_name_type(data)?.add(" "))
        } else {
            Ok(String::default())
        }
    }

    pub fn place_description_die(data: Arc<UnitArchipelago>) -> Result<String, Error> {
        Ok(
            format!("It's a {}. {}{}",
                data.lexical_unit_lagoon.place_object.unit_pool[prelude::random_index(data.lexical_unit_lagoon.place_object.unit_pool.len())?],
                relative_type(data.clone())?,
                fame_type(data.clone())?
            )
        )
    }
    
    fn fame_type(data: Arc<UnitArchipelago>) -> Result<String, Error> {
        if prelude::random_from_range(0, 2)? == 0 {
            if prelude::random_from_range(0, 1)? == 0 {
                Ok(
                    format!(" It's known for it's {}.",
                        data.lexical_unit_lagoon.place_noun.unit_pool[prelude::random_index(data.lexical_unit_lagoon.place_noun.unit_pool.len())?]
                    )
                )
            } else {
                Ok(
                    format!(" It's known for it's {}.",
                        data.lexical_unit_lagoon.place_top_eng_nouns.unit_pool[prelude::random_index(data.lexical_unit_lagoon.place_top_eng_nouns.unit_pool.len())?]
                    )
                )
            }
        } else {
            Ok(String::default())
        }
    }

    fn relative_type(data: Arc<UnitArchipelago>) -> Result<String, Error> {
        if prelude::random_from_range(0, 2)? == 0 {
            Ok(
                format!("It's {} normal.",
                    data.lexical_unit_lagoon.place_relatives.unit_pool[prelude::random_index(data.lexical_unit_lagoon.place_relatives.unit_pool.len())?]
                )
            )
        } else {
            Ok(String::default())
        }
    }
    
}
