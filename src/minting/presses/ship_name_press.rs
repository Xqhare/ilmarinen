use std::io::Error;

use fortuna::Fortuna;

use crate::unit_pools::UnitArchipelago;

use self::ship_name::{ship_name_die, ship_prefix_die};

/// `ship_prefix` == "" => random prefix; " " == empty prefix; anything else == custom prefix
pub fn ship_name_press(
    data: &UnitArchipelago,
    rng: &mut Fortuna,
    ship_prefix: &str,
) -> Result<String, Error> {
    Ok(format!(
        "{} {}",
        ship_prefix_die(data, rng, ship_prefix)?,
        ship_name_die(data, rng)?
    ))
}

mod ship_name {
    use std::io::Error;

    use fortuna::{Fortuna, RngApi};

    use crate::{
        minting::presses::common::{
            artifact_adjective_type, formal_last_name_die, formal_name_die, random_abc_type,
            random_place_noun_type,
        },
        unit_pools::UnitArchipelago,
    };

    /// `ship_prefix` == "" => random prefix; " " == empty prefix; anything else == custom prefix
    pub fn ship_prefix_die(
        data: &UnitArchipelago,
        rng: &mut Fortuna,
        ship_prefix: &str,
    ) -> Result<String, Error> {
        match ship_prefix {
            "" => {
                if let Ok(n) = rng.random_from_range(0, 1)
                    && n == 0
                {
                    ship_prefix_type(data, rng).map(|s| s.to_string())
                } else if let Ok(i) = rng.random_from_range(0, 2) {
                    match i {
                        0 => ship_random_prefix_type(data, rng, 2),
                        1 => ship_random_prefix_type(data, rng, 3),
                        _ => ship_random_prefix_type(data, rng, 4),
                    }
                } else {
                    Ok(String::new())
                }
            }
            " " => Ok(String::new()),
            _ => Ok(ship_prefix.to_string()),
        }
    }

    pub fn ship_name_die(data: &UnitArchipelago, rng: &mut Fortuna) -> Result<String, Error> {
        if let Ok(n) = rng.random_from_range(0, 4) {
            match n {
                0 => formal_name_die(data, rng).map(|s| s.to_string()),
                1 => formal_last_name_die(data, rng).map(|s| s.to_string()),
                2 => ship_long_name_type(data, rng).map(|s| s.to_string()),
                3 => {
                    let mut random_noun = random_place_noun_type(data, rng)?.to_string();
                    let first_letter = random_noun.remove(0).to_uppercase();
                    random_noun.insert_str(0, first_letter.to_string().as_str());
                    Ok(random_noun)
                }
                _ => {
                    let mut random_noun = random_place_noun_type(data, rng)?.to_string();
                    let first_letter = random_noun.remove(0).to_uppercase();
                    random_noun.insert_str(0, first_letter.to_string().as_str());
                    Ok(format!(
                        "{} {}",
                        artifact_adjective_type(data, rng)?,
                        random_noun,
                    ))
                }
            }
        } else {
            Err(Error::other("ship_name_die failed"))
        }
    }

    fn ship_prefix_type<'a>(
        data: &'a UnitArchipelago,
        rng: &mut Fortuna,
    ) -> Result<&'a str, Error> {
        if let Ok(idx) = rng.random_index(data.lexical_unit_lagoon.ship_prefixes.unit_pool.len()) {
            Ok(&data.lexical_unit_lagoon.ship_prefixes.unit_pool[idx])
        } else {
            Err(Error::other("ship_prefix_type failed"))
        }
    }

    fn ship_random_prefix_type(
        data: &UnitArchipelago,
        rng: &mut Fortuna,
        letter_amount: usize,
    ) -> Result<String, Error> {
        let mut out: String = Default::default();
        for _ in 0..letter_amount {
            out.push_str(&random_abc_type(data, rng)?.to_uppercase());
        }
        Ok(out)
    }

    fn ship_long_name_type<'a>(
        data: &'a UnitArchipelago,
        rng: &mut Fortuna,
    ) -> Result<&'a str, Error> {
        if let Ok(idx) = rng.random_index(data.lexical_unit_lagoon.ship_long_names.unit_pool.len())
        {
            Ok(&data.lexical_unit_lagoon.ship_long_names.unit_pool[idx])
        } else {
            Err(Error::other("ship_long_name_type failed"))
        }
    }
}
