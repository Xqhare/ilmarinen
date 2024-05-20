use std::{sync::Arc, io::Error};

use crate::unit_pools::UnitArchipelago;

use self::ship_name::{ship_prefix_die, ship_name_die};

/// ship_prefix == "" => random prefix; " " == empty prefix; anything else == custom prefix
pub fn ship_name_press(data: Arc<UnitArchipelago>, ship_prefix: &str) -> Result<String, Error> {
    Ok(
        format!("{} {}",
            ship_prefix_die(data.clone(), ship_prefix)?,
            ship_name_die(data)?
        )
    )
}

mod ship_name {
    use std::{sync::Arc, io::Error};
    
    use tyche::prelude::{self, random_from_range};

    use crate::{unit_pools::UnitArchipelago, minting::presses::common::{random_abc_type, formal_name_die, formal_last_name_die, random_place_noun_type, artifact_adjective_type}};

    /// ship_prefix == "" => random prefix; " " == empty prefix; anything else == custom prefix
    pub fn ship_prefix_die(data: Arc<UnitArchipelago>, ship_prefix: &str) -> Result<String, Error> {
        match ship_prefix {
            "" => {
                if random_from_range(0, 1)? == 0 {
                    ship_prefix_type(data)
                } else {
                    match random_from_range(0, 2)? {
                        0 => ship_random_prefix_type(data, 2), 
                        1 => ship_random_prefix_type(data, 3),
                        _ => ship_random_prefix_type(data, 4),
                    }
                }
            },
            " " => Ok("".to_string()),
            _ => Ok(ship_prefix.to_string()),
        }
    }

    pub fn ship_name_die(data: Arc<UnitArchipelago>) -> Result<String, Error> {
        match random_from_range(0, 4)? {
            0 => formal_name_die(data),
            1 => formal_last_name_die(data),
            2 => ship_long_name_type(data),
            3 => {
                let mut random_noun = random_place_noun_type(data)?;
                let first_letter = random_noun.remove(0).to_uppercase();
                random_noun.insert_str(0, first_letter.to_string().as_str());
                Ok( random_noun )
            },
            _ => {
                let mut random_noun = random_place_noun_type(data.clone())?;
                let first_letter = random_noun.remove(0).to_uppercase();
                random_noun.insert_str(0, first_letter.to_string().as_str());
                Ok(
                    format!("{} {}",
                        artifact_adjective_type(data)?,
                        random_noun,
                    )
                )
            },
        }
    }

    fn ship_prefix_type(data: Arc<UnitArchipelago>) -> Result<String, Error> {
        Ok(
            data.lexical_unit_lagoon.ship_prefixes.unit_pool[prelude::random_index(data.lexical_unit_lagoon.ship_prefixes.unit_pool.len())?].clone()
        )
    }

    fn ship_random_prefix_type(data: Arc<UnitArchipelago>, letter_amount: usize) -> Result<String, Error> {
        let mut out: String = Default::default();
        for _n in 0..letter_amount {
            out.push_str(&random_abc_type(data.clone())?.to_uppercase());
        }
        Ok (out)
    }

    fn ship_long_name_type(data: Arc<UnitArchipelago>) -> Result<String, Error> {
        Ok(
            data.lexical_unit_lagoon.ship_long_names.unit_pool[prelude::random_index(data.lexical_unit_lagoon.ship_long_names.unit_pool.len())?].clone()
        )
    }
}
