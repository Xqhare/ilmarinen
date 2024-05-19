use std::{sync::Arc, io::Error};

use tyche::prelude;

use crate::unit_pools::UnitArchipelago;

use self::empire::empire_name_die;

use super::{common::{formal_name_die, language_name_die, full_place_name_die, general_currency_name_die}, government_press::government_press};

pub fn empire_press(data: Arc<UnitArchipelago>) -> Result<String, Error> {
    let name = empire_name_die(data.clone())?;
    let area = prelude::random_from_range(1, 10000000)?;
    let pop = prelude::random_from_range(1, 1000000)?;
    Ok(
        format!("Name: {}; Leader: {}; Capital: {}; Official language: {}; Population: {}; Area: {}; Currency: {}; Government: {}",
        name,
        formal_name_die(data.clone())?,
        full_place_name_die(data.clone())?,
        language_name_die(data.clone())?,
        pop,
        area,
        general_currency_name_die(data.clone())?,
        government_press(data, &name)?
        )
    )
}

mod empire {
    use std::{io::Error, sync::Arc};

    use tyche::prelude;

    use crate::{unit_pools::UnitArchipelago, minting::presses::common::{simple_name_type, comp_name_type}};

    pub fn empire_name_die(data: Arc<UnitArchipelago>) -> Result<String, Error> {
        if prelude::random_from_range(0, 1)? == 0 {
            Ok(
                format!("{} {}",
                simple_name_type(data.clone())?,
                simple_name_type(data)?
                )
            )
        } else {
            Ok(comp_name_type(data)?)
        }
    }

}
