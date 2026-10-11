use std::{io::Error, sync::Arc};

use tyche::prelude;

use crate::unit_pools::UnitArchipelago;

use self::metal_alloy::{alloy_die, metal_die};

pub fn metal_alloy_press(data: &Arc<UnitArchipelago>) -> Result<String, Error> {
    if prelude::random_from_range(0, 1)? == 0 {
        metal_die(data).map(|x| x.to_string())
    } else {
        alloy_die(data)
    }
}

mod metal_alloy {
    use std::{io::Error, sync::Arc};

    use tyche::prelude;

    use crate::unit_pools::UnitArchipelago;

    pub fn metal_die(data: &Arc<UnitArchipelago>) -> Result<&str, Error> {
        Ok(&data.lexical_unit_lagoon.metals_list.unit_pool
            [prelude::random_index(data.lexical_unit_lagoon.metals_list.unit_pool.len())?])
    }

    pub fn alloy_die(data: &Arc<UnitArchipelago>) -> Result<String, Error> {
        let main_metal = metal_die(data)?;
        let seed = prelude::random_from_range(0, 2)?;
        if seed == 0 {
            Ok(format!("{}-{} Alloy", main_metal, metal_die(data)?))
        } else if seed == 1 {
            Ok(format!("{}-{} Alloy", main_metal, metal_alloy_type(data)?))
        } else {
            let newseed = prelude::random_from_range(0, 2)?;
            if newseed == 2 {
                Ok(format!(
                    "{}-{}-{} Alloy",
                    main_metal,
                    metal_die(data)?,
                    metal_die(data)?
                ))
            } else if newseed == 1 {
                Ok(format!(
                    "{}-{}-{} Alloy",
                    main_metal,
                    metal_die(data)?,
                    metal_alloy_type(data)?
                ))
            } else {
                Ok(format!(
                    "{}-{}-{} Alloy",
                    main_metal,
                    metal_alloy_type(data)?,
                    metal_alloy_type(data)?
                ))
            }
        }
    }

    fn metal_alloy_type(data: &Arc<UnitArchipelago>) -> Result<&str, Error> {
        Ok(&data.lexical_unit_lagoon.metals_alloys_list.unit_pool
            [prelude::random_index(data.lexical_unit_lagoon.metals_alloys_list.unit_pool.len())?])
    }
}
