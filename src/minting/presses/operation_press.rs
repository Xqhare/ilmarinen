use std::{sync::Arc, io::Error};

use crate::unit_pools::UnitArchipelago;

use self::operation::name_die;

pub fn operation_press(data: Arc<UnitArchipelago>) -> Result<String, Error> {
    Ok(
        format!("Operation {}", name_die(data)?)
    )
}

mod operation {
    use std::{sync::Arc, io::Error};
    
    use tyche::prelude;

    use crate::{unit_pools::UnitArchipelago, minting::presses::common::{full_place_name_die, comp_name_type, nickname_type, first_name_type, last_name_type, skill_die, artifact_type_type, artifact_adjective_type, material_type, quality_type}};

    pub fn name_die(data: Arc<UnitArchipelago>) -> Result<String, Error> {
        if prelude::random_from_range(0, 1)? == 0 {
            obj_type(data)
        } else {
            Ok(
                format!("{} {}",
                    adj_type(data.clone())?,
                    obj_type(data)?
                )
            )
        }
    }

    fn obj_type(data: Arc<UnitArchipelago>) -> Result<String, Error> {
        let seed = prelude::random_from_range(0, 6)?;
        match seed {
            0 => return full_place_name_die(data),
            1 => return comp_name_type(data),
            2 => return nickname_type(data),
            3 => return first_name_type(data),
            4 => return last_name_type(data),
            5 => return skill_die(data),
            _ => return artifact_type_type(data),
        }
    }

    fn adj_type(data: Arc<UnitArchipelago>) -> Result<String, Error> {
        let seed = prelude::random_from_range(0, 3)?;
        match seed {
            0 => return Ok(data.lexical_unit_lagoon.place_single.unit_pool[prelude::random_index(data.lexical_unit_lagoon.place_single.unit_pool.len())?].clone()),
            1 => return artifact_adjective_type(data),
            2 => return material_type(data),
            _ => return quality_type(data)
        }
    }

}

