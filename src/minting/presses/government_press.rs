use std::{sync::Arc, io::Error};

use crate::unit_pools::UnitArchipelago;

use self::government::government_form_die;

use super::common::empire_name_type;

pub fn government_press(data: Arc<UnitArchipelago>, empire_name: &str) -> Result<String, Error> {
    match empire_name {
        "" => Ok(
        format!("{} of {}",
            government_form_die(data.clone())?,
            empire_name_type(data)?
        )
    ),
        _ => Ok(
            format!("{} of {}",
                government_form_die(data.clone())?,
                empire_name
            ),
        )
    }
}

mod government {
    use std::{io::Error, sync::Arc};

    use tyche::prelude;

    use crate::unit_pools::UnitArchipelago;

    pub fn government_form_die(data: Arc<UnitArchipelago>) -> Result<String, Error> {
        let seed = prelude::random_from_range(0, 2)?;
        if seed == 0 {
            Ok(
                format!("{} {} {}",
                    government_name0_type(data.clone())?,
                    government_name1_type(data)?,
                    government_type_type()?
                )
            )

        } else if seed == 1 {
            Ok(
                format!("Semi-{} {}",
                    government_name0_type(data)?,
                    government_type_type()?
                )
            )
        } else {
            Ok(
                format!("{} Monarchy",
                    government_name_monarchy_type(data)?
                )
            )
        }
    }

    pub fn government_type_type() -> Result<String, Error> {
        let seed = prelude::random_from_range(0, 3)?;
        if seed == 0 {
            Ok("State".to_string())
        } else if seed == 1 {
            Ok("Republic".to_string())
        } else if seed == 2 {
            Ok("Commonwealth".to_string())
        } else {
            Ok("Confederacy".to_string())
        }
    }

    pub fn government_name0_type(data: Arc<UnitArchipelago>) -> Result<String, Error> {
        Ok(
            data.lexical_unit_lagoon.government_name0.unit_pool[prelude::random_index(data.lexical_unit_lagoon.government_name0.unit_pool.len())?].clone()
        )
    }

    pub fn government_name1_type(data: Arc<UnitArchipelago>) -> Result<String, Error> {
        Ok(
            data.lexical_unit_lagoon.government_name1.unit_pool[prelude::random_index(data.lexical_unit_lagoon.government_name1.unit_pool.len())?].clone()
        )
    }

    pub fn government_name_monarchy_type(data: Arc<UnitArchipelago>) -> Result<String, Error> {
        Ok(
            data.lexical_unit_lagoon.government_name_monarchy.unit_pool[prelude::random_index(data.lexical_unit_lagoon.government_name_monarchy.unit_pool.len())?].clone()
        )
    }

}
