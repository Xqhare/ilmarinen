use std::{sync::Arc, io::Error};

use crate::unit_pools::UnitArchipelago;

use self::artifact::{material_type, artifact_type_type, name_type, quality_type, art_type, dedication_type};

use super::common::comp_name_type;

pub fn artifact_press(data: Arc<UnitArchipelago>) -> Result<String, Error> {
    let artifact_type = artifact_type_type(data.clone())?;
    let mat0 = material_type(data.clone())?;
    let mat1 = material_type(data.clone())?;
    let mat2 = material_type(data.clone())?;
    let mat_full = format!("{}, {} and {}", mat0, mat1, mat2);
    let native_name = comp_name_type(data.clone())?;
    Ok(
        format!("This is {}, it is known in its native tounge as {}. It is a {} made of {} and is of {} quality. It shows {} in {} with highlights in {}. It also shows {} in {}. It is dedicated to {}.",
            name_type(data.clone())?,
            native_name,
            artifact_type,
            mat_full,
            quality_type(data.clone())?,
            art_type(data.clone())?,
            mat1,
            mat2,
            art_type(data.clone())?,
            material_type(data.clone())?,
            dedication_type(data)?
        )
    )
}

mod artifact {
    use std::{sync::Arc, io::Error};
    
    use tyche::prelude;

    use crate::{unit_pools::UnitArchipelago, minting::presses::common::comp_name_type};

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

    pub fn art_type(data: Arc<UnitArchipelago>) -> Result<String, Error> {
        Ok(
            data.lexical_unit_lagoon.artifact_art.unit_pool[prelude::random_index(data.lexical_unit_lagoon.artifact_art.unit_pool.len())?].clone()
        )
    }

    pub fn dedication_type(data: Arc<UnitArchipelago>) -> Result<String, Error> {
        Ok(
            data.lexical_unit_lagoon.artifact_dedication.unit_pool[prelude::random_index(data.lexical_unit_lagoon.artifact_dedication.unit_pool.len())?].clone()
        )
    }

    pub fn name_type(data: Arc<UnitArchipelago>) -> Result<String, Error> {
        Ok(
            format!("{} {}",
                adjective_type(data.clone())?,
                comp_name_type(data)?
            )
        )
    }
}

