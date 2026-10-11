use std::io::Error;

use fortuna::Fortuna;

use crate::unit_pools::UnitArchipelago;

use self::ship_class::{ship_fame_press, ship_size_type, ship_tech_die, ship_type_type};

use super::ship_name_press::ship_name_press;

/// Creates a String containing the generated ship class.
///
/// Arguments:
///
/// * `ship_size`: &str, ship size as found in `ship_sizes` inside the `UnitArchipelago`
/// * `avg_speed`: f32, the average speed relative to light-speed. 0.5 == light-speed*0.5; 1 == lightspeed
/// * `avg_range`: f32, the average range in light-years
///
/// # Returns
/// A String containing the ship class
///
/// # Errors
/// Io errors
pub fn ship_class_press(
    data: &UnitArchipelago,
    rng: &mut Fortuna,
    ship_size: &str,
    avg_speed: f32,
    avg_range: f32,
) -> Result<String, Error> {
    let ship_name = ship_name_press(data, rng, "")?;
    if ship_size.is_empty() || ship_size == " " {
        let ship_size_generated = ship_size_type(data, rng)?;
        let ship_type = ship_type_type(data, rng, &ship_size_generated)?;
        let ship_tech = ship_tech_die(rng, &ship_size_generated, avg_speed, avg_range)?;
        let ship_fame = ship_fame_press(data, rng, &ship_size_generated)?;
        Ok(format!(
            "{ship_name}-Class: It's a {ship_type}. {ship_tech} {ship_fame}"
        ))
    } else {
        let ship_type = ship_type_type(data, rng, ship_size)?;
        let ship_tech = ship_tech_die(rng, ship_size, avg_speed, avg_range)?;
        let ship_fame = ship_fame_press(data, rng, ship_size)?;
        Ok(format!(
            "{ship_name}-Class: It's a {ship_type}. {ship_tech} {ship_fame}"
        ))
    }
}

mod ship_class {
    use std::{io::Error, ops::Mul};

    use fortuna::{Fortuna, RngApi};

    use crate::unit_pools::UnitArchipelago;

    pub fn ship_tech_die(
        rng: &mut Fortuna,
        ship_size: &str,
        avg_speed: f32,
        avg_range: f32,
    ) -> Result<String, Error> {
        let (
            tonnage_min,
            tonnage_max,
            length_min,
            length_max,
            speed_min,
            speed_max,
            range_min,
            range_max,
            ftl_type,
            crew_min_low,
            crew_min_high,
            crew_max_high,
        ) = match ship_size {
            "SS" => (1.0, 1000.0, 1.0, 100.0, 0.0, 0.33, 0.33, 0.75, 1, 0, 3, 10),
            "XS" => (
                1000.0, 2000.0, 100.0, 175.0, 0.95, 1.05, 0.0, 0.33, 1, 1, 12, 25,
            ),
            "S" => (
                2000.0, 5000.0, 175.0, 350.0, 0.95, 1.10, 0.95, 1.10, 2, 5, 25, 50,
            ),
            "M" => (
                5000.0, 10000.0, 350.0, 675.0, 1.05, 1.15, 1.05, 1.10, 3, 15, 175, 350,
            ),
            "L" => (
                10000.0, 15000.0, 675.0, 1000.0, 1.15, 1.25, 1.10, 1.33, 3, 25, 400, 800,
            ),
            "XL" => (
                15000.0, 25000.0, 1000.0, 2500.0, 1.25, 1.40, 1.33, 1.50, 6, 50, 500, 1200,
            ),
            "XXL" => (
                25000.0, 75000.0, 2500.0, 6000.0, 1.4, 1.6, 1.5, 1.65, 8, 150, 500, 3000,
            ),
            "U" => (
                75000.0, 150000.0, 6000.0, 10000.0, 1.5, 1.6, 1.5, 1.65, 10, 300, 800, 8000,
            ),
            "XU" => (
                150000.0, 500000.0, 10000.0, 15000.0, 1.6, 1.65, 1.4, 1.55, 20, 800, 1500, 14000,
            ),
            "T" => (
                500000.0,
                1000000000.0,
                15000.0,
                15000000.0,
                1.6,
                1.7,
                1.35,
                1.75,
                25,
                1000,
                2000,
                1400000,
            ),
            _ => {
                return Err(Error::other(format!(
                    "Unable to match supplied `ship_size` ({ship_size:?}). No match matching {ship_size:?}"
                )));
            }
        };

        let tonnage: f32 = if let Ok(n) = rng.random_from_f32_range(tonnage_min, tonnage_max) {
            n
        } else {
            return Err(Error::other("Unable to generate ship tonnage".to_string()));
        };
        let length_m: f32 = if let Ok(n) = rng.random_from_f32_range(length_min, length_max) {
            n
        } else {
            return Err(Error::other("Unable to generate ship length".to_string()));
        };
        let speed_ls: f32 = if let Ok(n) = rng.random_from_f32_range(speed_min, speed_max) {
            avg_speed.mul(n)
        } else {
            return Err(Error::other("Unable to generate ship speed".to_string()));
        };
        let range_ly: String =
            ship_ftl_capability_type(rng, avg_range, range_min, range_max, ftl_type)?;
        let crew_min: usize = if let Ok(n) = rng.random_from_range(crew_min_low, crew_min_high) {
            n
        } else {
            return Err(Error::other("Unable to generate ship crew min".to_string()));
        };
        let crew_max: usize = if let Ok(n) = rng.random_from_range(crew_min, crew_max_high) {
            n
        } else {
            return Err(Error::other("Unable to generate ship crew max".to_string()));
        };

        Ok(format!(
            "Tonnage: {tonnage:.2}. Length (m): {length_m:.0}. Min. landing pad size: {ship_size}. Speed (c): {speed_ls:.2}. Range (ly): {range_ly}. Minimum needed crew: {crew_min}. Maximum crew: {crew_max}."
        ))
    }

    pub fn ship_fame_press(
        data: &UnitArchipelago,
        rng: &mut Fortuna,
        ship_size: &str,
    ) -> Result<String, Error> {
        match ship_size {
            "L" => ship_fame_die(data, rng, 1),
            "XL" => ship_fame_die(data, rng, 1),
            "XXL" => {
                if let Ok(n) = rng.random_from_range(0, 1) {
                    match n {
                        0 => ship_fame_die(data, rng, 1),
                        _ => ship_fame_die(data, rng, 2),
                    }
                } else {
                    Err(Error::other("Unable to generate ship fame".to_string()))
                }
            }
            "U" => {
                if let Ok(n) = rng.random_from_range(0, 2) {
                    match n {
                        0 => ship_fame_die(data, rng, 1),
                        1 => ship_fame_die(data, rng, 2),
                        _ => ship_fame_die(data, rng, 3),
                    }
                } else {
                    Err(Error::other("Unable to generate ship fame".to_string()))
                }
            }
            "XU" => {
                if let Ok(n) = rng.random_from_range(0, 3) {
                    match n {
                        0 => ship_fame_die(data, rng, 1),
                        1 => ship_fame_die(data, rng, 2),
                        2 => ship_fame_die(data, rng, 3),
                        _ => ship_fame_die(data, rng, 4),
                    }
                } else {
                    Err(Error::other("Unable to generate ship fame".to_string()))
                }
            }
            "T" => {
                if let Ok(n) = rng.random_from_range(0, 4) {
                    match n {
                        0 => ship_fame_die(data, rng, 1),
                        1 => ship_fame_die(data, rng, 2),
                        2 => ship_fame_die(data, rng, 3),
                        3 => ship_fame_die(data, rng, 4),
                        _ => ship_fame_die(data, rng, 5),
                    }
                } else {
                    Err(Error::other("Unable to generate ship fame".to_string()))
                }
            }
            _ => Ok(String::default()),
        }
    }

    pub fn ship_fame_die(
        data: &UnitArchipelago,
        rng: &mut Fortuna,
        famous_attribute_amount: usize,
    ) -> Result<String, Error> {
        let mut out: String = Default::default();
        for attribute in 0..famous_attribute_amount {
            if attribute == 0 {
                out.push_str(&format!(
                    "The Class gained notoriety for it's {}",
                    ship_fame_type(data, rng)?
                ));
            } else if attribute == famous_attribute_amount.saturating_sub(1) {
                out.push_str(&format!(" and it's {}", ship_fame_type(data, rng)?));
            } else {
                out.push_str(&format!(", it's {}", ship_fame_type(data, rng)?));
            }
        }
        out.push('.');
        Ok(out)
    }

    pub fn ship_fame_type<'a>(
        data: &'a UnitArchipelago,
        rng: &mut Fortuna,
    ) -> Result<&'a str, Error> {
        if let Ok(idx) = rng.random_index(data.lexical_unit_lagoon.ship_fame.unit_pool.len()) {
            Ok(&data.lexical_unit_lagoon.ship_fame.unit_pool[idx])
        } else {
            Err(Error::other("ship_fame_type failed"))
        }
    }

    pub fn ship_type_type<'a>(
        data: &'a UnitArchipelago,
        rng: &mut Fortuna,
        ship_size: &str,
    ) -> Result<&'a str, Error> {
        let unit_pool = match ship_size {
            "SS" => &data.lexical_unit_lagoon.ship_type_ss.unit_pool,
            "XS" => &data.lexical_unit_lagoon.ship_type_xs.unit_pool,
            "S" => &data.lexical_unit_lagoon.ship_type_s.unit_pool,
            "M" => &data.lexical_unit_lagoon.ship_type_m.unit_pool,
            "L" => &data.lexical_unit_lagoon.ship_type_l.unit_pool,
            "XL" => &data.lexical_unit_lagoon.ship_type_xl.unit_pool,
            "XXL" => &data.lexical_unit_lagoon.ship_type_xxl.unit_pool,
            "U" => &data.lexical_unit_lagoon.ship_type_u.unit_pool,
            "XU" => &data.lexical_unit_lagoon.ship_type_xu.unit_pool,
            "T" => &data.lexical_unit_lagoon.ship_type_t.unit_pool,
            _ => {
                return Err(Error::other(format!(
                    "Unable to match supplied `ship_size` ({ship_size:?}). No arcm matching {ship_size:?}"
                )));
            }
        };
        if let Ok(idx) = rng.random_index(unit_pool.len()) {
            Ok(&unit_pool[idx])
        } else {
            Err(Error::other("ship_type_type failed"))
        }
    }

    pub fn ship_size_type<'a>(
        data: &'a UnitArchipelago,
        rng: &mut Fortuna,
    ) -> Result<&'a str, Error> {
        if let Ok(idx) = rng.random_index(data.lexical_unit_lagoon.ship_sizes.unit_pool.len()) {
            Ok(&data.lexical_unit_lagoon.ship_sizes.unit_pool[idx])
        } else {
            Err(Error::other("ship_size_type failed"))
        }
    }

    fn ship_ftl_capability_type(
        rng: &mut Fortuna,
        avg_range: f32,
        lower_speed_bound: f32,
        upper_speed_bound: f32,
        ftl_chance: usize,
    ) -> Result<String, Error> {
        if let Ok(n) = rng.random_from_range(0, ftl_chance)
            && n == 0
        {
            Ok("No FTL capability".to_string())
        } else if let Ok(comp0) = rng.random_from_f32_range(lower_speed_bound, upper_speed_bound)
            && let Ok(comp1) = rng.random_from_f32_range(lower_speed_bound, upper_speed_bound)
        {
            let range = avg_range.mul_add(comp0, comp1);
            Ok(format!("{range:.2}"))
        } else {
            Err(Error::other(
                "Unable to generate ship FTL capability".to_string(),
            ))
        }
    }
}
