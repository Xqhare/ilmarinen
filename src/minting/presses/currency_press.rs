use std::{sync::Arc, io::Error};

use crate::unit_pools::UnitArchipelago;

use self::currency::{currency_abbreviation_type, non_decimal_currency_system_press};

use super::common::general_currency_name_die;

pub fn currency_press(data: Arc<UnitArchipelago>, decimal_nondecimal_time_based_choice: &str) -> Result<String, Error> {
    let currency_name = general_currency_name_die(data.clone())?;
    Ok(
        format!("{}",
        non_decimal_currency_system_press(data, currency_name)?,
        )
    )
}

mod currency {
    use std::{sync::Arc, io::Error};

    use tyche::prelude::{random_index, random_from_range};
    use unicode_segmentation::UnicodeSegmentation;

    use crate::{unit_pools::UnitArchipelago, minting::presses::common::general_currency_name_die};

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
    
    pub fn non_decimal_currency_system_die(data: Arc<UnitArchipelago>, currency_name: String, unit_amount: usize) -> Result<String, Error> {
        let abbr = currency_abbreviation_type(currency_name.clone())?;
        let mut out: Vec<String> = vec![format!("The currency is called {currency_name} and abbreviated as {abbr}.")];
        // Main unit
        out.push(format!("The main unit is called {currency_name}"));
        out.push(currency_coin_denomination_and_material_press(data.clone(), &currency_name)?);
        // Safety check!
        if unit_amount > data.numerical_unit_lagoon.currency_non_decimal_base.unit_pool.len() {
            return Err(Error::other(format!("Only {} different coin values in database. Requested {} different coin values.", data.numerical_unit_lagoon.currency_non_decimal_base.unit_pool.len(), unit_amount)));
        }
        // Rest unit generation
        if unit_amount > 0 {
            let mut bases = data.numerical_unit_lagoon.currency_non_decimal_base.unit_pool.clone();
            let mut last_unit_name: String = Default::default();
            for unit in 0..unit_amount {
                if unit == 0 {
                    // Super unit
                    let super_unit_name = general_currency_name_die(data.clone())?;
                    let super_base = bases.remove(random_index(bases.len())?);
                    out.push(format!("The super-unit is called {}.", super_unit_name));
                    out.push(currency_coin_denomination_and_material_press(data.clone(), &super_unit_name)?);
                    out.push(format!("The super unit exchanges for 1 {} = {} {}.", super_unit_name, super_base, currency_name.clone()));
                    last_unit_name = currency_name.clone();
                } else {
                    // {unit}. sub-unit
                    let sub_unit_name = general_currency_name_die(data.clone())?;
                    let sub_base = bases.remove(random_index(bases.len())?);
                    out.push(format!("The {}. sub-unit is called {}.", unit, sub_unit_name));
                    out.push(currency_coin_denomination_and_material_press(data.clone(), &sub_unit_name)?);
                    out.push(format!("The sub-unit exchanges for 1 {} = {} {}", sub_unit_name, sub_base, last_unit_name));
                    last_unit_name = sub_unit_name;
                }
            }
        }
        Ok(out.join("\n"))
    }

    pub fn currency_coin_denomination_and_material_press(data: Arc<UnitArchipelago>, unit_name: &str) -> Result<String, Error> {
        Ok(format!("! TODO -- TODO !"))
    }

    pub fn non_decimal_currency_base_type(data: Arc<UnitArchipelago>) -> Result<String, Error> {
        Ok(
            data.numerical_unit_lagoon.currency_non_decimal_base.unit_pool[random_index(data.numerical_unit_lagoon.currency_non_decimal_base.unit_pool.len())?].clone().to_string()
        )
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

}
