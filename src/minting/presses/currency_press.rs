use std::{sync::Arc, io::Error};

use tyche::prelude::random_from_range;

use crate::unit_pools::UnitArchipelago;

use self::currency::{non_decimal_currency_system_press, decimal_currency_system_press, time_currency_system_press};

use super::common::general_currency_name_die;

/// # Arguments
/// * `decimal_nondecimal_time_based_choice`: &str. "" / _ = random, "decimal" / "time" / "nondecimal" for fine controll
pub fn currency_press(data: Arc<UnitArchipelago>, decimal_nondecimal_time_based_choice: &str) -> Result<String, Error> {
    let currency_name = general_currency_name_die(data.clone())?;
    match decimal_nondecimal_time_based_choice {
        "decimal" => decimal_currency_system_press(data, currency_name),
        "time" => time_currency_system_press(data, currency_name),
        "nondecimal" => non_decimal_currency_system_press(data, currency_name),
        _ => {
            match random_from_range(0, 2)? {
                0 => decimal_currency_system_press(data, currency_name),
                1 => time_currency_system_press(data, currency_name),
                _ => non_decimal_currency_system_press(data, currency_name),
            }
        }
    }
}

mod currency {
    use std::{sync::Arc, io::Error};

    use tyche::prelude::{random_index, random_from_range};
    use unicode_segmentation::UnicodeSegmentation;

    use crate::{unit_pools::UnitArchipelago, minting::{presses::{common::{general_currency_name_die, random_abc_type, formal_name_die, simple_name_type, full_people_name_die}, metal_alloy_press::metal_alloy_press}, minters::mint_metal_alloy}};

    pub fn non_decimal_currency_system_press(data: Arc<UnitArchipelago>, currency_name: String) -> Result<String, Error> {
        match random_from_range(0, 5)? {
            0 => non_decimal_currency_system_die(data, currency_name, 0),
            1 => non_decimal_currency_system_die(data, currency_name, 1),
            2 => non_decimal_currency_system_die(data, currency_name, 2),
            3 => non_decimal_currency_system_die(data, currency_name, 3),
            4 => non_decimal_currency_system_die(data, currency_name, 4),
            _ => non_decimal_currency_system_die(data, currency_name, 5),
        }
    }
 
    pub fn decimal_currency_system_press(data: Arc<UnitArchipelago>, currency_name: String) -> Result<String, Error> {
        match random_from_range(0, 5)? {
            0 => decimal_currency_system_die(data, currency_name, 0),
            1 => decimal_currency_system_die(data, currency_name, 1),
            2 => decimal_currency_system_die(data, currency_name, 2),
            3 => decimal_currency_system_die(data, currency_name, 3),
            4 => decimal_currency_system_die(data, currency_name, 4),
            _ => decimal_currency_system_die(data, currency_name, 5),
        }
    }
 
    pub fn time_currency_system_press(data: Arc<UnitArchipelago>, currency_name: String) -> Result<String, Error> {
        match random_from_range(0, 5)? {
            0 => time_currency_system_die(data, currency_name, 0),
            1 => time_currency_system_die(data, currency_name, 1),
            2 => time_currency_system_die(data, currency_name, 2),
            3 => time_currency_system_die(data, currency_name, 3),
            4 => time_currency_system_die(data, currency_name, 4),
            _ => time_currency_system_die(data, currency_name, 5),
        }
    }

    pub fn decimal_currency_system_die(data: Arc<UnitArchipelago>, currency_name: String, unit_amount: usize) -> Result<String, Error> {
        let abbr = currency_abbreviation_type(currency_name.clone())?;
        let mut out: Vec<String> = vec![format!("This is a decimal currency."), format!("The currency is called {currency_name} and abbreviated as {abbr}.")];
        // Main unit
        out.push(format!("The main unit is called {currency_name}"));
        out.push(currency_coin_denomination_and_material_die(data.clone(), &currency_name)?);
        // Rest unit generation
        if unit_amount > 0 {
            let mut last_unit_name: String = currency_name.clone();
            let mut first_sub_unit_taken = false;
            for unit in 0..unit_amount {
                if unit == 0 {
                    // Super unit
                    // It's rare in real life so 1 in 10 should suffice.
                    if random_from_range(0, 9)? == 0 {
                        let super_unit_name = general_currency_name_die(data.clone())?;
                        out.insert(2, format!("The super-unit is called {}.", super_unit_name));
                        out.insert(3, currency_coin_denomination_and_material_die(data.clone(), &super_unit_name)?);
                        out.insert(4, format!("The super unit {} exchanges for 1 {} = 100 {}.", super_unit_name, super_unit_name, currency_name.clone()));
                    } else {
                        // another sub unit instead, remember unit == 0!
                        let sub_unit_name = general_currency_name_die(data.clone())?;
                        out.push(format!("The 1. sub-unit is called {}.", sub_unit_name));
                        out.push(currency_coin_denomination_and_material_die(data.clone(), &sub_unit_name)?);
                        out.push(format!("The 1. sub-unit {} exchanges for 100 {} = 1 {}", sub_unit_name, sub_unit_name, last_unit_name));
                        last_unit_name = sub_unit_name;
                        first_sub_unit_taken = true;
                    }
                } else {
                    // {unit}. sub-unit
                    let actual_number = {
                        if first_sub_unit_taken {
                            unit.saturating_add(1)
                        } else {
                            unit
                        }
                    };
                    let sub_unit_name = general_currency_name_die(data.clone())?;
                    out.push(format!("The {}. sub-unit is called {}.", actual_number, sub_unit_name));
                    out.push(currency_coin_denomination_and_material_die(data.clone(), &sub_unit_name)?);
                    out.push(format!("The {}. sub-unit {} exchanges for 100 {} = 1 {}", actual_number, sub_unit_name, sub_unit_name, last_unit_name));
                    last_unit_name = sub_unit_name;
                }
            }
        }
        Ok(out.join("\n"))
    }

    pub fn non_decimal_currency_system_die(data: Arc<UnitArchipelago>, currency_name: String, unit_amount: usize) -> Result<String, Error> {
        let abbr = currency_abbreviation_type(currency_name.clone())?;
        let mut out: Vec<String> = vec![format!("This is a non decimal currency."), format!("The currency is called {currency_name} and abbreviated as {abbr}.")];
        // Main unit
        out.push(format!("The main unit is called {currency_name}"));
        out.push(currency_coin_denomination_and_material_die(data.clone(), &currency_name)?);
        // Safety check!
        if unit_amount > data.numerical_unit_lagoon.currency_non_decimal_base.unit_pool.len() {
            return Err(Error::other(format!("Only {} different coin values in database. Requested {} different coin values.", data.numerical_unit_lagoon.currency_non_decimal_base.unit_pool.len(), unit_amount)));
        }
        // Rest unit generation
        if unit_amount > 0 {
            let mut bases = data.numerical_unit_lagoon.currency_non_decimal_base.unit_pool.clone();
            let mut last_unit_name: String = currency_name.clone();
            let mut first_sub_unit_taken = false;
            for unit in 0..unit_amount {
                if unit == 0 {
                    // Super unit
                    // It's rare in real life so 1 in 10 should suffice.
                    if random_from_range(0, 9)? == 0 {
                        let super_unit_name = general_currency_name_die(data.clone())?;
                        let super_base = bases.remove(random_index(bases.len())?);
                        out.insert(2, format!("The super-unit is called {}.", super_unit_name));
                        out.insert(3, currency_coin_denomination_and_material_die(data.clone(), &super_unit_name)?);
                        out.insert(4, format!("The super unit {} exchanges for 1 {} = {} {}.", super_unit_name, super_unit_name, super_base, currency_name.clone()));
                    } else {
                        // another sub unit instead, remember unit == 0!
                        let sub_unit_name = general_currency_name_die(data.clone())?;
                        let sub_base = bases.remove(random_index(bases.len())?);
                        out.push(format!("The 1. sub-unit is called {}.", sub_unit_name));
                        out.push(currency_coin_denomination_and_material_die(data.clone(), &sub_unit_name)?);
                        out.push(format!("The 1. sub-unit {} exchanges for {} {} = 1 {}", sub_unit_name, sub_base, sub_unit_name, last_unit_name));
                        last_unit_name = sub_unit_name;
                        first_sub_unit_taken = true;
                    }
                } else {
                    // {unit}. sub-unit
                    let actual_number = {
                        if first_sub_unit_taken {
                            unit.saturating_add(1)
                        } else {
                            unit
                        }
                    };
                    let sub_unit_name = general_currency_name_die(data.clone())?;
                    let sub_base = bases.remove(random_index(bases.len())?);
                    out.push(format!("The {}. sub-unit is called {}.", actual_number, sub_unit_name));
                    out.push(currency_coin_denomination_and_material_die(data.clone(), &sub_unit_name)?);
                    out.push(format!("The {}. sub-unit {} exchanges for {} {} = 1 {}", actual_number, sub_unit_name, sub_base, sub_unit_name, last_unit_name));
                    last_unit_name = sub_unit_name;
                }
            }
        }
        Ok(out.join("\n"))
    }

    pub fn time_currency_system_die(data: Arc<UnitArchipelago>, currency_name: String, unit_amount: usize) -> Result<String, Error> {
        let abbr = currency_abbreviation_type(currency_name.clone())?;
        let mut out: Vec<String> = vec![format!("This is a time based currency."), format!("The currency is called {currency_name} and abbreviated as {abbr}.")];
        // Main unit
        out.push(format!("The main unit is called {currency_name} and 1 {currency_name} is equal to {} hours of work for the good of society.", currency_time_fractions_type(data.clone())?));
        out.push(currency_coin_denomination_and_material_die(data.clone(), &currency_name)?);
        // Rest unit generation
        if unit_amount > 0 {
            let mut bases = data.numerical_unit_lagoon.currency_time_fractions.unit_pool.clone();
            // Takes out the `1`, as exchanging 1 for 1 is stupid. Assumes that 1 is
            // first in json library.
            let _ = bases.remove(0);
            let mut last_unit_name: String = currency_name.clone();
            let mut first_sub_unit_taken = false;
            for unit in 0..unit_amount {
                if unit == 0 {
                    // Super unit
                    // It's rare in real life so 1 in 10 should suffice.
                    if random_from_range(0, 9)? == 0 {
                        let super_unit_name = general_currency_name_die(data.clone())?;
                        let super_base = bases.remove(random_index(bases.len())?);
                        out.insert(2, format!("The super-unit is called {}.", super_unit_name));
                        out.insert(3, currency_coin_denomination_and_material_die(data.clone(), &super_unit_name)?);
                        out.insert(4, format!("The super unit {} exchanges for 1 {} = {} {}.", super_unit_name, super_unit_name, super_base, currency_name.clone()));
                    } else {
                        // another sub unit instead, remember unit == 0!
                        let sub_unit_name = general_currency_name_die(data.clone())?;
                        let sub_base = bases.remove(random_index(bases.len())?);
                        out.push(format!("The 1. sub-unit is called {}.", sub_unit_name));
                        out.push(currency_coin_denomination_and_material_die(data.clone(), &sub_unit_name)?);
                        out.push(format!("The 1. sub-unit {} exchanges for {} {} = 1 {}", sub_unit_name, sub_base, sub_unit_name, last_unit_name));
                        last_unit_name = sub_unit_name;
                        first_sub_unit_taken = true;
                    }
                } else {
                    // {unit}. sub-unit
                    let actual_number = {
                        if first_sub_unit_taken {
                            unit.saturating_add(1)
                        } else {
                            unit
                        }
                    };
                    let sub_unit_name = general_currency_name_die(data.clone())?;
                    let sub_base = bases.remove(random_index(bases.len())?);
                    out.push(format!("The {}. sub-unit is called {}.", actual_number, sub_unit_name));
                    out.push(currency_coin_denomination_and_material_die(data.clone(), &sub_unit_name)?);
                    out.push(format!("The {}. sub-unit {} exchanges for {} {} = 1 {}", actual_number, sub_unit_name, sub_base, sub_unit_name, last_unit_name));
                    last_unit_name = sub_unit_name;
                }
            }
        }
        Ok(out.join("\n"))
    }

    pub fn currency_coin_denomination_and_material_die(data: Arc<UnitArchipelago>, unit_name: &str) -> Result<String, Error> {
        let coin_num = random_from_range(1, 7)?;
        let mut denominations: Vec<String> = Default::default();
        for _ in 0..coin_num {
            denominations.push(currency_coin_denomination_type(data.clone())?);
        }
        let mut out: Vec<String> = vec![format!("The {} has the 1 coin. The coin {}. {}.", unit_name, currency_material_type(data.clone())?, currency_coin_iconography_press(data.clone())?)];
        for denomination in denominations {
            out.push(
                format!("The {denomination} coin, {}. {}.", currency_material_type(data.clone())?, currency_coin_iconography_press(data.clone())?)
            )

        }
        Ok(out.join("\n"))
    }

    pub fn currency_coin_iconography_press(data: Arc<UnitArchipelago>) -> Result<String, Error> {
        let out = vec![format!("It shows {} on it's front,", currency_coin_iconography_die(data.clone(), false)?), format!("and it shows {} on it's back", currency_coin_iconography_die(data, true)?)];
        Ok(out.join("\n"))
    }

    pub fn currency_coin_iconography_die(data: Arc<UnitArchipelago>, reverse: bool) -> Result<String, Error> {
        let seed = {
            if reverse {
                4
            } else {
                3
            }
        };
        match random_from_range(0, seed)? {
            0 => {
                currency_icon_inscription_type(data)
            },
            1 => {
            },
            2 => {
            },
            3 => {
            },
            _ => {
            },
        }
    }

    pub fn currency_icon_figure_type(data: Arc<UnitArchipelago>) -> Result<String, Error> {
    }

    pub fn currency_icon_inscription_type(data: Arc<UnitArchipelago>) -> Result<String, Error> {
        let type_to_check = currency_inscription_type(data.clone())?;
        match type_to_check.as_str() {
            "Letter" => Ok(
                            format!("the letter {}", random_abc_type(data)?.to_uppercase())
                        ),
            "Monogram" => Ok(
                            format!("a monogram of the letters {} and {}", random_abc_type(data.clone())?.to_uppercase(), random_abc_type(data)?.to_uppercase())
                        ),
            "Name" => {
                if random_from_range(0, 1)? == 0 {
                    formal_name_die(data)
                } else {
                    full_people_name_die(data)
                }
            },
            _ => {
                Err(Error::other(format!("{} from json library is not implemented in icon_inscription_type!", type_to_check)))
            },
        }

    }

    pub fn currency_material_type(data: Arc<UnitArchipelago>) -> Result<String, Error> {
        if random_from_range(0, 1)? == 0 {
            // mono metallic coin
            Ok(
                format!("is made of {}", metal_alloy_press(data.clone())?)
            )
        } else {
            // bi metallic coin
            Ok(
                format!("is made with a {} center plug and a {} outer ring", metal_alloy_press(data.clone())?, metal_alloy_press(data.clone())?)
            )
        }
    }

    pub fn currency_abbreviation_type(currency_name: String) -> Result<String, Error> {
        if random_from_range(0, 1)? == 0 {
            if currency_name.len() >= 3 {
                let mut tmp_name_bind = UnicodeSegmentation::graphemes(currency_name.as_str(), true).collect::<Vec<&str>>();
                let first_letter = tmp_name_bind.remove(0);
                let letter1 = tmp_name_bind.remove(random_index(tmp_name_bind.len())?);
                let letter2 = tmp_name_bind.remove(random_index(tmp_name_bind.len())?);
                Ok(
                    format!("{}{}{}",
                    first_letter.to_uppercase(),
                    letter1,
                    letter2
                    )
                )
            } else {
                Err(Error::other(format!("Name ({}) is shorter than 3 letters!", currency_name)))
            }
        } else {
            if currency_name.len() >= 3 {
                let mut tmp_name_bind = UnicodeSegmentation::graphemes(currency_name.as_str(), true).collect::<Vec<&str>>();
                let first_letter = tmp_name_bind.remove(0);
                let letter1 = tmp_name_bind.remove(0);
                let letter2 = tmp_name_bind.remove(0);
                Ok(
                    format!("{}{}{}",
                    first_letter.to_uppercase(),
                    letter1,
                    letter2
                    )
                )
            } else {
                Err(Error::other(format!("Name ({}) is shorter than 3 letters!", currency_name)))
            }
        }
        
    }

    pub fn currency_time_fractions_type(data: Arc<UnitArchipelago>) -> Result<String, Error> {
        Ok(
            data.numerical_unit_lagoon.currency_time_fractions.unit_pool[random_index(data.numerical_unit_lagoon.currency_time_fractions.unit_pool.len())?].clone().to_string()
        )
    }

    pub fn currency_coin_denomination_type(data: Arc<UnitArchipelago>) -> Result<String, Error> {
        Ok(
            data.numerical_unit_lagoon.currency_coins_denomination.unit_pool[random_index(data.numerical_unit_lagoon.currency_coins_denomination.unit_pool.len())?].clone().to_string()
        )
    }

    pub fn currency_inscription_type(data: Arc<UnitArchipelago>) -> Result<String, Error> {
        Ok(
            data.lexical_unit_lagoon.currency_icon_inscription.unit_pool[random_index(data.lexical_unit_lagoon.currency_icon_inscription.unit_pool.len())?].clone()
        )
    }

}
