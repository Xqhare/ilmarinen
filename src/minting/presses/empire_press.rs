use std::io::Error;

use fortuna::{Fortuna, RngApi};

use crate::unit_pools::UnitArchipelago;

use self::empire::empire_name_die;

use super::{
    common::{formal_name_die, full_place_name_die, general_currency_name_die, language_name_die},
    government_press::government_press,
};

pub fn empire_press(data: &UnitArchipelago, rng: &mut Fortuna) -> Result<String, Error> {
    let name = empire_name_die(data, rng)?;
    let (area, pop) = if let Ok(area) = rng.random_from_range(1, 10_000_000)
        && let Ok(pop) = rng.random_from_range(1, 100_000_000)
    {
        (area, pop)
    } else {
        return Err(Error::other(
            "empire_press failed to generate population and area",
        ));
    };
    Ok(format!(
        "Name: {}; Leader: {}; Capital: {}; Official language: {}; Population: {}; Area: {}; Currency: {}; Government: {}",
        name,
        formal_name_die(data, rng)?,
        full_place_name_die(data, rng)?,
        language_name_die(data, rng)?,
        pop,
        area,
        general_currency_name_die(data, rng)?,
        government_press(data, rng, &name)?
    ))
}

mod empire {
    use std::io::Error;

    use crate::{
        minting::presses::common::{comp_name_type, simple_name_type},
        unit_pools::UnitArchipelago,
    };
    use fortuna::{Fortuna, RngApi};

    pub fn empire_name_die(data: &UnitArchipelago, rng: &mut Fortuna) -> Result<String, Error> {
        if let Ok(n) = rng.random_from_range(0, 1)
            && n == 0
        {
            Ok(format!(
                "{} {}",
                simple_name_type(data, rng)?,
                simple_name_type(data, rng)?
            ))
        } else {
            Ok(comp_name_type(data, rng)?)
        }
    }
}
