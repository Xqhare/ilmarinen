use std::io::Error;

use crate::unit_pools::UnitArchipelago;

use self::artifact::{art_type, dedication_type, name_type};
use fortuna::Fortuna;

use super::common::{artifact_type_type, comp_name_type, material_type, quality_type};

pub fn artifact_press(data: &UnitArchipelago, rng: &mut Fortuna) -> Result<String, Error> {
    let artifact_type = artifact_type_type(data, rng)?;
    let mat0 = material_type(data, rng)?;
    let mat1 = material_type(data, rng)?;
    let mat2 = material_type(data, rng)?;
    let mat_full = format!("{mat0}, {mat1} and {mat2}");
    let native_name = comp_name_type(data, rng)?;
    Ok(format!(
        "This is {}, it is known in its native tounge as {}. It is a {} made of {} and is of {} quality. It shows {} in {} with highlights in {}. It also shows {} in {}. It is dedicated to {}.",
        name_type(data, rng)?,
        native_name,
        artifact_type,
        mat_full,
        quality_type(data, rng)?,
        art_type(data, rng)?,
        mat1,
        mat2,
        art_type(data, rng)?,
        material_type(data, rng)?,
        dedication_type(data, rng)?
    ))
}

mod artifact {
    use std::io::Error;

    use crate::{
        minting::presses::common::{artifact_adjective_type, comp_name_type},
        unit_pools::UnitArchipelago,
    };
    use fortuna::{Fortuna, RngApi};

    pub fn art_type<'a>(data: &'a UnitArchipelago, rng: &mut Fortuna) -> Result<&'a str, Error> {
        if let Ok(idx) = rng.random_index(data.lexical_unit_lagoon.artifact_art.unit_pool.len()) {
            Ok(&data.lexical_unit_lagoon.artifact_art.unit_pool[idx])
        } else {
            Err(Error::other("art_type failed"))
        }
    }

    pub fn dedication_type<'a>(
        data: &'a UnitArchipelago,
        rng: &mut Fortuna,
    ) -> Result<&'a str, Error> {
        if let Ok(idx) =
            rng.random_index(data.lexical_unit_lagoon.artifact_dedication.unit_pool.len())
        {
            Ok(&data.lexical_unit_lagoon.artifact_dedication.unit_pool[idx])
        } else {
            Err(Error::other("dedication_type failed"))
        }
    }

    pub fn name_type(data: &UnitArchipelago, rng: &mut Fortuna) -> Result<String, Error> {
        Ok(format!(
            "{} {}",
            artifact_adjective_type(data, rng)?,
            comp_name_type(data, rng)?
        ))
    }
}
