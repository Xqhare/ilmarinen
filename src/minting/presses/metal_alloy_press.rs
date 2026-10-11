use std::io::Error;

use fortuna::{Fortuna, RngApi};

use crate::unit_pools::UnitArchipelago;

use self::metal_alloy::{alloy_die, metal_die};

pub fn metal_alloy_press(data: &UnitArchipelago, rng: &mut Fortuna) -> Result<String, Error> {
    if let Ok(n) = rng.random_from_range(0, 2)
        && n == 0
    {
        metal_die(data, rng).map(|x| x.to_string())
    } else {
        alloy_die(data, rng)
    }
}

mod metal_alloy {
    use std::io::Error;

    use fortuna::{Fortuna, RngApi};

    use crate::unit_pools::UnitArchipelago;

    pub fn metal_die<'a>(data: &'a UnitArchipelago, rng: &mut Fortuna) -> Result<&'a str, Error> {
        if let Ok(idx) = rng.random_index(data.lexical_unit_lagoon.metals_list.unit_pool.len()) {
            Ok(&data.lexical_unit_lagoon.metals_list.unit_pool[idx])
        } else {
            Err(Error::other("metal_die failed"))
        }
    }

    pub fn alloy_die(data: &UnitArchipelago, rng: &mut Fortuna) -> Result<String, Error> {
        let main_metal = metal_die(data, rng)?;
        if let Ok(seed) = rng.random_from_range(0, 2) {
            if seed == 0 {
                Ok(format!("{}-{} Alloy", main_metal, metal_die(data, rng)?))
            } else if seed == 1 {
                Ok(format!(
                    "{}-{} Alloy",
                    main_metal,
                    metal_alloy_type(data, rng)?
                ))
            } else {
                let newseed = rng.random_from_range(0, 2).unwrap_or(0);
                if newseed == 2 {
                    Ok(format!(
                        "{}-{}-{} Alloy",
                        main_metal,
                        metal_die(data, rng)?,
                        metal_die(data, rng)?
                    ))
                } else if newseed == 1 {
                    Ok(format!(
                        "{}-{}-{} Alloy",
                        main_metal,
                        metal_die(data, rng)?,
                        metal_alloy_type(data, rng)?
                    ))
                } else {
                    Ok(format!(
                        "{}-{}-{} Alloy",
                        main_metal,
                        metal_alloy_type(data, rng)?,
                        metal_alloy_type(data, rng)?
                    ))
                }
            }
        } else {
            Err(Error::other("alloy_die failed"))
        }
    }

    fn metal_alloy_type<'a>(
        data: &'a UnitArchipelago,
        rng: &mut Fortuna,
    ) -> Result<&'a str, Error> {
        if let Ok(idx) =
            rng.random_index(data.lexical_unit_lagoon.metals_alloys_list.unit_pool.len())
        {
            Ok(&data.lexical_unit_lagoon.metals_alloys_list.unit_pool[idx])
        } else {
            Err(Error::other("metal_alloy_type failed"))
        }
    }
}
