use std::io::Error;

use fortuna::Fortuna;

use crate::unit_pools::UnitArchipelago;

use self::place::{adj_die, place_description_die};

use super::common::full_place_name_die;

pub fn place_press(data: &UnitArchipelago, rng: &mut Fortuna) -> Result<String, Error> {
    Ok(format!(
        "{}{}. {}",
        adj_die(data, rng)?,
        full_place_name_die(data, rng)?,
        place_description_die(data, rng)?
    ))
}

mod place {
    use std::{
        io::{Error, ErrorKind},
        ops::Add,
    };

    use fortuna::{Fortuna, RngApi};

    use crate::{minting::presses::common::simple_name_type, unit_pools::UnitArchipelago};

    pub fn adj_die(data: &UnitArchipelago, rng: &mut Fortuna) -> Result<String, Error> {
        if let Ok(n) = rng.random_from_range(0, 1)
            && n == 0
        {
            Ok(simple_name_type(data, rng)?.add(" "))
        } else {
            Ok(String::default())
        }
    }

    pub fn place_description_die(
        data: &UnitArchipelago,
        rng: &mut Fortuna,
    ) -> Result<String, Error> {
        if let Ok(n) = rng.random_index(data.lexical_unit_lagoon.place_object.unit_pool.len()) {
            Ok(format!(
                "It's a {}. {}{}",
                data.lexical_unit_lagoon.place_object.unit_pool[n],
                relative_type(data, rng)?,
                fame_type(data, rng)?
            ))
        } else {
            Err(Error::new(ErrorKind::Other, "Place description error"))
        }
    }

    fn fame_type(data: &UnitArchipelago, rng: &mut Fortuna) -> Result<String, Error> {
        if let Ok(n) = rng.random_from_range(0, 2)
            && n == 0
        {
            if let Ok(i) = rng.random_from_range(0, 1)
                && i == 0
                && let Ok(idx) =
                    rng.random_index(data.lexical_unit_lagoon.place_noun.unit_pool.len())
            {
                Ok(format!(
                    " It's known for it's {}.",
                    data.lexical_unit_lagoon.place_noun.unit_pool[idx]
                ))
            } else if let Ok(idx) =
                rng.random_index(data.lexical_unit_lagoon.place_top_eng_nouns.unit_pool.len())
            {
                Ok(format!(
                    " It's known for it's {}.",
                    data.lexical_unit_lagoon.place_top_eng_nouns.unit_pool[idx]
                ))
            } else {
                Ok(String::default())
            }
        } else {
            Ok(String::default())
        }
    }

    fn relative_type(data: &UnitArchipelago, rng: &mut Fortuna) -> Result<String, Error> {
        if let Ok(n) = rng.random_from_range(0, 2)
            && n == 0
            && let Ok(idx) =
                rng.random_index(data.lexical_unit_lagoon.place_relatives.unit_pool.len())
        {
            Ok(format!(
                "It's {} normal.",
                data.lexical_unit_lagoon.place_relatives.unit_pool[idx]
            ))
        } else {
            Ok(String::default())
        }
    }
}
