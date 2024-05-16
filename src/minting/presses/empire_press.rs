use std::{sync::Arc, io::Error};

use crate::unit_pools::UnitArchipelago;

pub fn empire_press(data: Arc<UnitArchipelago>) -> Result<String, Error> {
    Ok(
        format!("{}{}. {}",
        "",
        "",
        ""
        )
    )
}

mod empire {
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

}
