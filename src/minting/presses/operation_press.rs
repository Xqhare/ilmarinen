use std::io::Error;

use fortuna::Fortuna;

use crate::{minting::minting_type::ObjectResult, unit_pools::UnitArchipelago};

use self::operation::name_die;

pub fn operation_press(data: &UnitArchipelago, rng: &mut Fortuna) -> Result<String, Error> {
    Ok(format!("Operation {}", name_die(data, rng)?))
}

pub fn operation_object_press(
    data: &UnitArchipelago,
    rng: &mut Fortuna,
) -> Result<ObjectResult, Error> {
    Ok(ObjectResult::OperationObject(name_die(data, rng)?))
}

mod operation {
    use fortuna::{Fortuna, RngApi};
    use std::io::Error;

    use crate::{
        minting::presses::{
            common::{
                artifact_adjective_type, artifact_type_type, comp_name_type, first_name_type,
                full_place_name_die, last_name_type, material_type, nickname_type, quality_type,
            },
            metal_alloy_press::metal_alloy_press,
        },
        unit_pools::UnitArchipelago,
    };

    pub fn name_die(data: &UnitArchipelago, rng: &mut Fortuna) -> Result<String, Error> {
        if let Ok(n) = rng.random_from_range(0, 1)
            && n == 0
        {
            obj_type(data, rng)
        } else {
            Ok(format!("{} {}", adj_type(data, rng)?, obj_type(data, rng)?))
        }
    }

    fn obj_type(data: &UnitArchipelago, rng: &mut Fortuna) -> Result<String, Error> {
        if let Ok(seed) = rng.random_from_range(0, 6) {
            match seed {
                0 => full_place_name_die(data, rng),
                1 => comp_name_type(data, rng),
                2 => nickname_type(data, rng),
                3 => first_name_type(data, rng),
                4 => last_name_type(data, rng),
                5 => metal_alloy_press(data, rng),
                _ => artifact_type_type(&data, rng).map(|x| x.to_string()),
            }
        } else {
            Err(Error::other("obj_type failed"))
        }
    }

    fn adj_type<'a>(data: &'a UnitArchipelago, rng: &mut Fortuna) -> Result<&'a str, Error> {
        if let Ok(seed) = rng.random_from_range(0, 3) {
            match seed {
                0 => {
                    if let Ok(idx) =
                        rng.random_index(data.lexical_unit_lagoon.place_single.unit_pool.len())
                    {
                        Ok(&data.lexical_unit_lagoon.place_single.unit_pool[idx])
                    } else {
                        Err(Error::other("adj_type failed"))
                    }
                }
                1 => artifact_adjective_type(data, rng),
                2 => material_type(data, rng),
                _ => quality_type(data, rng),
            }
        } else {
            Err(Error::other("adj_type failed"))
        }
    }
}
