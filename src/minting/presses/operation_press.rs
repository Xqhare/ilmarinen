use std::{io::Error, sync::Arc};

use crate::unit_pools::UnitArchipelago;

use self::operation::name_die;

pub fn operation_press(data: Arc<UnitArchipelago>) -> Result<String, Error> {
    Ok(format!("Operation {}", name_die(data)?))
}

mod operation {
    use std::{io::Error, sync::Arc};

    use tyche::prelude;

    use crate::{
        minting::presses::common::{
            artifact_adjective_type, artifact_type_type, comp_name_type, first_name_type,
            full_place_name_die, last_name_type, material_type, nickname_type, quality_type,
            skill_die,
        },
        unit_pools::UnitArchipelago,
    };

    pub fn name_die(data: Arc<UnitArchipelago>) -> Result<String, Error> {
        if prelude::random_from_range(0, 1)? == 0 {
            obj_type(data)
        } else {
            Ok(format!("{} {}", adj_type(data.clone())?, obj_type(data)?))
        }
    }

    fn obj_type(data: Arc<UnitArchipelago>) -> Result<String, Error> {
        let seed = prelude::random_from_range(0, 6)?;
        match seed {
            0 => full_place_name_die(data),
            1 => comp_name_type(data),
            2 => nickname_type(data),
            3 => first_name_type(data),
            4 => last_name_type(data),
            5 => skill_die(data),
            _ => artifact_type_type(data),
        }
    }

    fn adj_type(data: Arc<UnitArchipelago>) -> Result<String, Error> {
        let seed = prelude::random_from_range(0, 3)?;
        match seed {
            0 => Ok(data.lexical_unit_lagoon.place_single.unit_pool
                [prelude::random_index(data.lexical_unit_lagoon.place_single.unit_pool.len())?]
            .clone()),
            1 => artifact_adjective_type(data),
            2 => material_type(data),
            _ => quality_type(data),
        }
    }
}
