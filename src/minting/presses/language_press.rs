use std::{sync::Arc, io::Error};

use crate::unit_pools::UnitArchipelago;

use super::common::language_name_die;

pub fn language_press(data: Arc<UnitArchipelago>) -> Result<String, Error> {
    language_name_die(data)
}

/* mod language {
    use std::{sync::Arc, io::Error};
    
    use tyche::prelude;

    use crate::{unit_pools::UnitArchipelago, minting::presses::common::{comp_name_type, simple_name_type}};

    
} */
