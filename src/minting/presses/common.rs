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

pub fn formal_name_die(data: Arc<UnitArchipelago>) -> Result<String, Error> {
    Ok(
        format!("{} {}",
            data.clone().lexical_unit_lagoon.people_title.unit_pool[prelude::random_index(data.lexical_unit_lagoon.people_title.unit_pool.len())?],
            legal_name_die(data)?
        )
    )
}

pub fn legal_name_die(data: Arc<UnitArchipelago>) -> Result<String, Error> {
    Ok(
        format!("{} {}",
            first_name_type(data.clone())?,
            last_name_type(data)?
        )
    )
}

pub fn simple_name_type(data: Arc<UnitArchipelago>) -> Result<String, Error> {
    let seed = prelude::random_from_range(0, 2)?;
    if seed >= 2 {
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

pub fn empire_name_type(data: Arc<UnitArchipelago>) -> Result<String, Error> {
    if prelude::random_from_range(0, 1)? == 1 {
        simple_name_type(data)
    } else {
        comp_name_type(data)
    }
}

pub fn full_place_name_die(data: Arc<UnitArchipelago>) -> Result<String, Error> {
    match prelude::random_from_range(0, 3)? {
        3 => simple_name_type(data),
        2 => Ok(
                format!("{} {}",
                simple_name_type(data.clone())?,
                simple_name_type(data)?
                )
            ),
        1 => Ok(
                format!("{}'s {}",
                    comp_name_type(data.clone())?,
                    simple_name_type(data)?
                )
            ),
        _ => comp_name_type(data),
    }
}

pub fn nickname_type(data: Arc<UnitArchipelago>) -> Result<String, Error> {
    Ok(
        format!("{}",
            if prelude::random_from_range(0, 2)? == 0 {
                data.lexical_unit_lagoon.people_nickname.unit_pool[prelude::random_index(data.lexical_unit_lagoon.people_nickname.unit_pool.len())?].clone()
            } else {
                String::default()
            }
        )
    )
}

pub fn first_name_type(data: Arc<UnitArchipelago>) -> Result<String, Error> {
    if prelude::random_from_range(0, 1)? == 0 {
        comp_name_type(data)
    } else {
        Ok(
            data.lexical_unit_lagoon.people_first_name.unit_pool[prelude::random_index(data.lexical_unit_lagoon.people_first_name.unit_pool.len())?].clone()
        )
    }
}

pub fn last_name_type(data: Arc<UnitArchipelago>) -> Result<String, Error> {
    if prelude::random_from_range(0, 1)? == 0 {
        comp_name_type(data)
    } else {
        Ok(
            data.lexical_unit_lagoon.people_last_name.unit_pool[prelude::random_index(data.lexical_unit_lagoon.people_last_name.unit_pool.len())?].clone()
        )
    }
}

pub fn skill_die(data: Arc<UnitArchipelago>) -> Result<String, Error> {
    Ok(
        format!("{} {}",
            data.lexical_unit_lagoon.people_skill_level.unit_pool[prelude::random_index(data.lexical_unit_lagoon.people_skill_level.unit_pool.len())?],
            data.lexical_unit_lagoon.people_skill.unit_pool[prelude::random_index(data.lexical_unit_lagoon.people_skill.unit_pool.len())?]
        )
    )
}

pub fn material_type(data: Arc<UnitArchipelago>) -> Result<String, Error> {
    Ok(
        data.lexical_unit_lagoon.artifact_material.unit_pool[prelude::random_index(data.lexical_unit_lagoon.artifact_material.unit_pool.len())?].clone()
    )
}

pub fn artifact_type_type(data: Arc<UnitArchipelago>) -> Result<String, Error> {
    Ok(
        data.lexical_unit_lagoon.artifact_type.unit_pool[prelude::random_index(data.lexical_unit_lagoon.artifact_type.unit_pool.len())?].clone()
    )
}

pub fn adjective_type(data: Arc<UnitArchipelago>) -> Result<String, Error> {
    Ok(
        data.lexical_unit_lagoon.artifact_adjective.unit_pool[prelude::random_index(data.lexical_unit_lagoon.artifact_adjective.unit_pool.len())?].clone()
    )
}

pub fn quality_type(data: Arc<UnitArchipelago>) -> Result<String, Error> {
    Ok(
        data.lexical_unit_lagoon.artifact_quality.unit_pool[prelude::random_index(data.lexical_unit_lagoon.artifact_quality.unit_pool.len())?].clone()
    )
}

pub fn language_name_die(data: Arc<UnitArchipelago>) -> Result<String, Error> {
    if prelude::random_from_range(0, 1)? == 0 {
        simple_name_type(data)
    } else {
        comp_name_type(data)
    }
}

