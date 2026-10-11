use std::{io::Error, sync::Arc};

use crate::unit_pools::UnitArchipelago;

use self::artifact::{art_type, dedication_type, name_type};

use super::common::{artifact_type_type, comp_name_type, material_type, quality_type};

pub fn artifact_press(data: &Arc<UnitArchipelago>) -> Result<String, Error> {
    let artifact_type = artifact_type_type(data)?;
    let mat0 = material_type(data)?;
    let mat1 = material_type(data)?;
    let mat2 = material_type(data)?;
    let mat_full = format!("{mat0}, {mat1} and {mat2}");
    let native_name = comp_name_type(data)?;
    Ok(
        format!("This is {}, it is known in its native tounge as {}. It is a {} made of {} and is of {} quality. It shows {} in {} with highlights in {}. It also shows {} in {}. It is dedicated to {}.",
            name_type(data)?,
            native_name,
            artifact_type,
            mat_full,
            quality_type(data)?,
            art_type(data)?,
            mat1,
            mat2,
            art_type(data)?,
            material_type(data)?,
            dedication_type(data)?
        )
    )
}

mod artifact {
    use std::{io::Error, sync::Arc};

    use tyche::prelude;

    use crate::{
        minting::presses::common::{artifact_adjective_type, comp_name_type},
        unit_pools::UnitArchipelago,
    };

    pub fn art_type(data: &Arc<UnitArchipelago>) -> Result<&str, Error> {
        Ok(&data.lexical_unit_lagoon.artifact_art.unit_pool
            [prelude::random_index(data.lexical_unit_lagoon.artifact_art.unit_pool.len())?])
    }

    pub fn dedication_type(data: &Arc<UnitArchipelago>) -> Result<&str, Error> {
        Ok(&data.lexical_unit_lagoon.artifact_dedication.unit_pool
            [prelude::random_index(data.lexical_unit_lagoon.artifact_dedication.unit_pool.len())?])
    }

    pub fn name_type(data: &Arc<UnitArchipelago>) -> Result<String, Error> {
        Ok(format!(
            "{} {}",
            artifact_adjective_type(data)?,
            comp_name_type(data)?
        ))
    }
}
