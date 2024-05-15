use std::{sync::Arc, io::Error};

use crate::unit_pools::UnitArchipelago;

use self::language::language_name_die;

pub fn language_press(data: Arc<UnitArchipelago>) -> Result<String, Error> {
    language_name_die(data)
}

mod language {
    use std::{sync::Arc, io::Error};
    
    use tyche::prelude;

    use crate::{unit_pools::UnitArchipelago, minting::presses::common::{comp_name_type, simple_name_type}};

    pub fn language_name_die(data: Arc<UnitArchipelago>) -> Result<String, Error> {
        if prelude::random_from_range(0, 1)? == 0 {
            simple_name_type(data)
        } else {
            comp_name_type(data)
        }
    }
}
