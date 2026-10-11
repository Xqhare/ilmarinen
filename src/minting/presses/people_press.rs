use std::io::Error;

use fortuna::Fortuna;

use crate::unit_pools::UnitArchipelago;

use self::people::trait_die;

use super::common::{full_people_name_die, skill_die};

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
        if let Ok(n) = rng.random_from_range(0, 1)
            && n == 0
            && let Ok(idx) =
                rng.random_index(data.lexical_unit_lagoon.people_trait_ck3.unit_pool.len())
        {
            Ok(&data.lexical_unit_lagoon.people_trait_ck3.unit_pool[idx])
        } else if let Ok(idx) =
            rng.random_index(data.lexical_unit_lagoon.people_trait_ck2.unit_pool.len())
        {
            Ok(&data.lexical_unit_lagoon.people_trait_ck2.unit_pool[idx])
        } else {
            Err(Error::other("trait_die failed"))
        }
    }
}
