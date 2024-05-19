use std::{sync::Arc, io::Error};

use crate::unit_pools::UnitArchipelago;

use self::currency::currency_;

pub fn currency_press(data: Arc<UnitArchipelago>) -> Result<String, Error> {
    Ok(
        format!("{}",
            currency_(data)?
        )
    )
}

mod currency {
    use std::{sync::Arc, io::Error};

    use tyche::prelude;

    use crate::unit_pools::UnitArchipelago;
    
    pub fn currency_(data: Arc<UnitArchipelago>) -> Result<String, Error> {
        if prelude::random_from_range(0, 1)? == 0 {
            Ok(String::default())
        } else {
            Ok(String::default())
        }
    }

}
