

pub mod common {
    use std::{sync::Arc, io::Error};

    use tyche::prelude;

    use crate::unit_pools::UnitArchipelago;

    pub fn comp_name_die(data: Arc<UnitArchipelago>) -> Result<String, Error> {
        Ok(format!("{}{}", data.lexical_unit_lagoon.general_comp0.unit_pool[prelude::random_index(data.lexical_unit_lagoon.general_comp0.unit_pool.len())?], data.lexical_unit_lagoon.general_comp1.unit_pool[prelude::random_index(data.lexical_unit_lagoon.general_comp1.unit_pool.len())?]))
    }
}


