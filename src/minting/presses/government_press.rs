use std::io::Error;

use fortuna::Fortuna;

use crate::unit_pools::UnitArchipelago;

use self::government::government_form_die;

use super::common::empire_name_type;

pub fn government_press(
    data: &UnitArchipelago,
    rng: &mut Fortuna,
    empire_name: &str,
) -> Result<String, Error> {
    match empire_name {
        "" => Ok(format!(
            "{} of {}",
            government_form_die(data, rng)?,
            empire_name_type(data, rng)?
        )),
        _ => Ok(format!(
            "{} of {}",
            government_form_die(data, rng)?,
            empire_name
        )),
    }
}

mod government {
    use std::io::Error;

    use fortuna::{Fortuna, RngApi};

    use crate::unit_pools::UnitArchipelago;

    pub fn government_form_die(data: &UnitArchipelago, rng: &mut Fortuna) -> Result<String, Error> {
        if let Ok(seed) = rng.random_from_range(0, 2) {
            if seed == 0 {
                Ok(format!(
                    "{} {} {}",
                    government_name0_type(data, rng)?,
                    government_name1_type(data, rng)?,
                    government_type_type(rng)?
                ))
            } else if seed == 1 {
                Ok(format!(
                    "Semi-{} {}",
                    government_name0_type(data, rng)?,
                    government_type_type(rng)?
                ))
            } else {
                Ok(format!(
                    "{} Monarchy",
                    government_name_monarchy_type(data, rng)?
                ))
            }
        } else {
            Err(Error::other("government_form_die failed"))
        }
    }

    pub fn government_type_type(rng: &mut Fortuna) -> Result<String, Error> {
        if let Ok(seed) = rng.random_from_range(0, 3) {
            if seed == 0 {
                Ok("State".to_string())
            } else if seed == 1 {
                Ok("Republic".to_string())
            } else if seed == 2 {
                Ok("Commonwealth".to_string())
            } else {
                Ok("Confederacy".to_string())
            }
        } else {
            Err(Error::other("government_type_type failed"))
        }
    }

    pub fn government_name0_type<'a>(
        data: &'a UnitArchipelago,
        rng: &mut Fortuna,
    ) -> Result<&'a str, Error> {
        if let Ok(idx) = rng.random_index(data.lexical_unit_lagoon.government_name0.unit_pool.len())
        {
            Ok(&data.lexical_unit_lagoon.government_name0.unit_pool[idx])
        } else {
            Err(Error::other("government_name0_type failed"))
        }
    }

    pub fn government_name1_type<'a>(
        data: &'a UnitArchipelago,
        rng: &mut Fortuna,
    ) -> Result<&'a str, Error> {
        if let Ok(idx) = rng.random_index(data.lexical_unit_lagoon.government_name1.unit_pool.len())
        {
            Ok(&data.lexical_unit_lagoon.government_name1.unit_pool[idx])
        } else {
            Err(Error::other("government_name1_type failed"))
        }
    }

    pub fn government_name_monarchy_type<'a>(
        data: &'a UnitArchipelago,
        rng: &mut Fortuna,
    ) -> Result<&'a str, Error> {
        if let Ok(idx) = rng.random_index(
            data.lexical_unit_lagoon
                .government_name_monarchy
                .unit_pool
                .len(),
        ) {
            Ok(&data.lexical_unit_lagoon.government_name_monarchy.unit_pool[idx])
        } else {
            Err(Error::other("government_name_monarchy_type failed"))
        }
    }
}
