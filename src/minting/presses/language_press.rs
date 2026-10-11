use std::io::Error;

use fortuna::Fortuna;

use crate::unit_pools::UnitArchipelago;

use super::common::language_name_die;

pub fn language_press(data: &UnitArchipelago, rng: &mut Fortuna) -> Result<String, Error> {
    language_name_die(data, rng)
}
