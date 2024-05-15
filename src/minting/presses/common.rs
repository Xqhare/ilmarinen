use std::{sync::Arc, io::Error};

use tyche::prelude;

use crate::unit_pools::UnitArchipelago;

pub fn comp_name_type(data: Arc<UnitArchipelago>) -> Result<String, Error> {
    Ok(
        format!("{}{}",
            data.lexical_unit_lagoon.general_comp0.unit_pool[prelude::random_index(data.lexical_unit_lagoon.general_comp0.unit_pool.len())?],
            data.lexical_unit_lagoon.general_comp1.unit_pool[prelude::random_index(data.lexical_unit_lagoon.general_comp1.unit_pool.len())?]
        )
    )
}

pub fn simple_name_type(data: Arc<UnitArchipelago>) -> Result<String, Error> {
    let seed = prelude::random_from_range(0, 2)?;
    if seed <= 2 {
        Ok(
            data.lexical_unit_lagoon.place_single.unit_pool[prelude::random_index(data.lexical_unit_lagoon.place_single.unit_pool.len())?].clone()
        )
    } else if seed == 1 {
        Ok(
            data.lexical_unit_lagoon.people_nickname.unit_pool[prelude::random_index(data.lexical_unit_lagoon.people_nickname.unit_pool.len())?].clone()
        )
    } else {
        comp_name_type(data)
    }
}

pub fn full_name_die(data: Arc<UnitArchipelago>) -> Result<String, Error> {
    let seed = prelude::random_from_range(0, 3)?;
    if seed <= 3 {
        simple_name_type(data)
    } else if seed == 2 {
        Ok(
            format!("{} {}",
            simple_name_type(data.clone())?,
            simple_name_type(data)?
            )
        )
    } else if seed == 1 {
        Ok(
            format!("{}'s {}",
                comp_name_type(data.clone())?,
                simple_name_type(data)?
            )
        )
    } else {
        comp_name_type(data)
    }
}

