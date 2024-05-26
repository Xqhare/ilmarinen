use std::{sync::Arc, io::Error};

use crate::unit_pools::UnitArchipelago;

use self::ship_class::{ship_size_type, ship_type_type, ship_tech_die, ship_fame_press};

use super::ship_name_press::ship_name_press;

/// Creates a String containing the generated ship class.
///
/// Arguments:
///
/// * `ship_size`: &str, ship size as found in ship_sizes inside the UnitArchipelago
/// * `avg_speed`: f32, the average speed relative to light-speed. 0.5 == light-speed*0.5; 1 == lightspeed
/// * `avg_range`: f32, the average range in light-years
///
/// # Returns
/// A String containing the ship class
///
/// # Errors
/// Io errors
pub fn ship_class_press(data: Arc<UnitArchipelago>, ship_size: &str, avg_speed: f32, avg_range: f32) -> Result<String, Error> {
    let ship_name = ship_name_press(data.clone(), "")?;
    if ship_size == "" || ship_size == " " {
        let ship_size_generated = ship_size_type(data.clone())?;
        let ship_type = ship_type_type(data.clone(), &ship_size_generated)?;
        let ship_tech = ship_tech_die(&ship_size_generated, avg_speed, avg_range)?;
        let ship_fame = ship_fame_press(data, &ship_size_generated)?;
        Ok(
            format!("{}-Class: It's a {}. {} {}",
            ship_name,
            ship_type,
            ship_tech,
            ship_fame
            )
        )
    } else {
        let ship_type = ship_type_type(data.clone(), ship_size)?;
        let ship_tech = ship_tech_die(ship_size, avg_speed, avg_range)?;
        let ship_fame = ship_fame_press(data, ship_size)?;
        Ok(
            format!("{}-Class: It's a {}. {} {}",
            ship_name,
            ship_type,
            ship_tech,
            ship_fame
            )
        )
    }
    
}

mod ship_class {
    use std::{sync::Arc, io::Error, ops::Mul};
    
    use tyche::prelude::{self, random_from_range, random_from_f32range};

    use crate::unit_pools::UnitArchipelago;

    #[allow(unused_assignments)]
    pub fn ship_tech_die(ship_size: &str, avg_speed: f32, avg_range: f32) -> Result<String, Error> {
        let mut tonnage: f32 = Default::default();
        let mut length_m: f32 = Default::default();
        let mut speed_ls: f32 = Default::default();
        let mut range_ly: String = Default::default();
        let mut crew_min: usize = Default::default();
        let mut crew_max: usize = Default::default();
        match ship_size {
            "SS" => {
                tonnage = random_from_f32range(1.0, 1000.0)?;
                length_m = random_from_f32range(1.0, 100.0)?;
                speed_ls = avg_speed.mul(random_from_f32range(0.0, 0.33)?);
                range_ly = ship_ftl_capability_type(avg_range, 0.33, 0.75, 1)?;
                crew_min = random_from_range(0, 3)?;
                crew_max = random_from_range(crew_min, 10)?;
            },
            "XS" => {
                tonnage = random_from_f32range(1000.0, 2000.0)?;
                length_m = random_from_f32range(100.0, 175.0)?;
                speed_ls = avg_speed.mul(random_from_f32range(0.95, 1.05)?);
                range_ly = ship_ftl_capability_type(avg_range, 0.0, 0.33, 1)?;
                crew_min = random_from_range(1, 12)?;
                crew_max = random_from_range(crew_min, 25)?;
            },
            "S" => {
                tonnage = random_from_f32range(2000.0, 5000.0)?;
                length_m = random_from_f32range(175.0, 350.0)?;
                speed_ls = avg_speed.mul(random_from_f32range(0.95, 1.10)?);
                range_ly = ship_ftl_capability_type(avg_range, 0.95, 1.10, 2)?;
                crew_min = random_from_range(5, 25)?;
                crew_max = random_from_range(crew_min, 50)?;
            },
            "M" => {
                tonnage = random_from_f32range(5000.0, 10000.0)?;
                length_m = random_from_f32range(350.0, 675.0)?;
                speed_ls = avg_speed.mul(random_from_f32range(1.05, 1.15)?);
                range_ly = ship_ftl_capability_type(avg_range, 1.05, 1.10, 3)?;
                crew_min = random_from_range(15, 175)?;
                crew_max = random_from_range(crew_min, 350)?;
            },
            "L" => {
                tonnage = random_from_f32range(10000.0, 15000.0)?;
                length_m = random_from_f32range(675.0, 1000.0)?;
                speed_ls = avg_speed.mul(random_from_f32range(1.15, 1.25)?);
                range_ly = ship_ftl_capability_type(avg_range, 1.10, 1.33, 3)?;
                crew_min = random_from_range(25, 400)?;
                crew_max = random_from_range(crew_min, 800)?;
            },
            "XL" => {
                tonnage = random_from_f32range(15000.0, 25000.0)?;
                length_m = random_from_f32range(1000.0, 2500.0)?;
                speed_ls = avg_speed.mul(random_from_f32range(1.25, 1.40)?);
                range_ly = ship_ftl_capability_type(avg_range, 1.33, 1.50, 6)?;
                crew_min = random_from_range(50, 500)?;
                crew_max = random_from_range(crew_min, 1200)?;
            },
            "XXL" => {
                tonnage = random_from_f32range(25000.0, 75000.0)?;
                length_m = random_from_f32range(2500.0, 6000.0)?;
                speed_ls = avg_speed.mul(random_from_f32range(1.4, 1.6)?);
                range_ly = ship_ftl_capability_type(avg_range, 1.5, 1.65, 8)?;
                crew_min = random_from_range(150, 500)?;
                crew_max = random_from_range(crew_min, 3000)?;
            },
            "U" => {
                tonnage = random_from_f32range(75000.0, 150000.0)?;
                length_m = random_from_f32range(6000.0, 10000.0)?;
                speed_ls = avg_speed.mul(random_from_f32range(1.5, 1.6)?);
                range_ly = ship_ftl_capability_type(avg_range, 1.5, 1.65, 10)?;
                crew_min = random_from_range(300, 800)?;
                crew_max = random_from_range(crew_min, 8000)?;
            },
            "XU" => {
                tonnage = random_from_f32range(150000.0, 500000.0)?;
                length_m = random_from_f32range(10000.0, 15000.0)?;
                speed_ls = avg_speed.mul(random_from_f32range(1.6, 1.65)?);
                range_ly = ship_ftl_capability_type(avg_range, 1.4, 1.55, 20)?;
                crew_min = random_from_range(800, 1500)?;
                crew_max = random_from_range(crew_min, 14000)?;    
            },
            "T" => {
                tonnage = random_from_f32range(500000.0, 1000000000.0)?;
                length_m = random_from_f32range(15000.0, 15000000.0)?;
                speed_ls = avg_speed.mul(random_from_f32range(1.6, 1.7)?);
                range_ly = ship_ftl_capability_type(avg_range, 1.35, 1.75, 25)?;
                crew_min = random_from_range(1000, 2000)?;
                crew_max = random_from_range(crew_min, 1400000)?;
            },
            _ => return Err(Error::other(format!("Unable to match supplied `ship_size` ({:?}). No arcm matching {:?}", ship_size, ship_size))),
        }
        Ok(
            format!("Tonnage: {:.2}. Length (m): {:.0}. Min. landing pad size: {}. Speed (c): {:.2}. Range (ly): {}. Minimum needed crew: {}. Maximum crew: {}.",
                tonnage,
                length_m,
                ship_size,
                speed_ls,
                range_ly,
                crew_min,
                crew_max
            )
        )
    }

    pub fn ship_fame_press(data: Arc<UnitArchipelago>, ship_size: &str) -> Result<String, Error> {
        match ship_size {
            "L" => ship_fame_die(data, 1),
            "XL" => ship_fame_die(data, 1),
            "XXL" => {
                match random_from_range(0, 1)? {
                    0 => ship_fame_die(data, 1),
                    _ => ship_fame_die(data, 2),
                }
            },
            "U" => {
                match random_from_range(0, 2)? {
                    0 => ship_fame_die(data, 1),
                    1 => ship_fame_die(data, 2),
                    _ => ship_fame_die(data, 3),
                }
            },
            "XU" => {
                match random_from_range(0, 3)? {
                    0 => ship_fame_die(data, 1),
                    1 => ship_fame_die(data, 2),
                    2 => ship_fame_die(data, 3),
                    _ => ship_fame_die(data, 4),
                }
            },
            "T" => {
                match random_from_range(0, 4)? {
                    0 => ship_fame_die(data, 1),
                    1 => ship_fame_die(data, 2),
                    2 => ship_fame_die(data, 3),
                    3 => ship_fame_die(data, 4),
                    _ => ship_fame_die(data, 5),
                }
            },
            _ => Ok(String::default()),
        }
    }

    pub fn ship_fame_die(data: Arc<UnitArchipelago>, famous_attribute_amount: usize) -> Result<String, Error> {
        let mut out: String = Default::default();
        for attribute in 0..famous_attribute_amount {
            if attribute == 0 {
                out.push_str(&format!("The Class gained notoriety for it's {}", ship_fame_type(data.clone())?));
            } else if attribute == famous_attribute_amount.saturating_sub(1) {
                out.push_str(&format!(" and it's {}", ship_fame_type(data.clone())?));
            } else {
                out.push_str(&format!(", it's {}", ship_fame_type(data.clone())?));
            }
        }
        out.push_str(".");
        Ok(out)
    }

    pub fn ship_fame_type(data: Arc<UnitArchipelago>) -> Result<String, Error> {
        Ok(
            data.lexical_unit_lagoon.ship_fame.unit_pool[prelude::random_index(data.lexical_unit_lagoon.ship_fame.unit_pool.len())?].clone()
        )
    }

    pub fn ship_type_type(data: Arc<UnitArchipelago>, ship_size: &str) -> Result<String, Error> {
        match ship_size {
            "SS" => Ok(
            data.lexical_unit_lagoon.ship_type_ss.unit_pool[prelude::random_index(data.lexical_unit_lagoon.ship_type_ss.unit_pool.len())?].clone()),
            "XS" => Ok(
            data.lexical_unit_lagoon.ship_type_xs.unit_pool[prelude::random_index(data.lexical_unit_lagoon.ship_type_xs.unit_pool.len())?].clone()),
            "S" => Ok(
            data.lexical_unit_lagoon.ship_type_s.unit_pool[prelude::random_index(data.lexical_unit_lagoon.ship_type_s.unit_pool.len())?].clone()),
            "M" => Ok(
            data.lexical_unit_lagoon.ship_type_m.unit_pool[prelude::random_index(data.lexical_unit_lagoon.ship_type_m.unit_pool.len())?].clone()),
            "L" => Ok(
            data.lexical_unit_lagoon.ship_type_l.unit_pool[prelude::random_index(data.lexical_unit_lagoon.ship_type_l.unit_pool.len())?].clone()),
            "XL" => Ok(
            data.lexical_unit_lagoon.ship_type_xl.unit_pool[prelude::random_index(data.lexical_unit_lagoon.ship_type_xl.unit_pool.len())?].clone()),
            "XXL" => Ok(
            data.lexical_unit_lagoon.ship_type_xxl.unit_pool[prelude::random_index(data.lexical_unit_lagoon.ship_type_xxl.unit_pool.len())?].clone()),
            "U" => Ok(
            data.lexical_unit_lagoon.ship_type_u.unit_pool[prelude::random_index(data.lexical_unit_lagoon.ship_type_u.unit_pool.len())?].clone()),
            "XU" => Ok(
            data.lexical_unit_lagoon.ship_type_xu.unit_pool[prelude::random_index(data.lexical_unit_lagoon.ship_type_xu.unit_pool.len())?].clone()),
            "T" => Ok(
            data.lexical_unit_lagoon.ship_type_t.unit_pool[prelude::random_index(data.lexical_unit_lagoon.ship_type_t.unit_pool.len())?].clone()),
            _ => Err(Error::other(format!("Unable to match supplied `ship_size` ({:?}). No arcm matching {:?}", ship_size, ship_size))),
        }
    }

    pub fn ship_size_type(data: Arc<UnitArchipelago>) -> Result<String, Error> {
        Ok(
            data.lexical_unit_lagoon.ship_sizes.unit_pool[prelude::random_index(data.lexical_unit_lagoon.ship_sizes.unit_pool.len())?].clone()
        )
    }

    fn ship_ftl_capability_type(avg_range: f32, lower_speed_bound: f32, upper_speed_bound: f32, ftl_chance: usize) -> Result<String, Error> {
        
        match random_from_range(0, ftl_chance)? {
            0 => Ok(format!("No FTL capability")),
            
            _ => {
                let range = avg_range.mul_add(random_from_f32range(lower_speed_bound, upper_speed_bound)?, random_from_f32range(lower_speed_bound, upper_speed_bound)?);
                Ok(format!("{:.2}", range))
            }
        }
    }
}
