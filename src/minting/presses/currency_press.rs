use std::io::Error;

use fortuna::{Fortuna, RngApi};
use thoth::Thoth;

use crate::unit_pools::UnitArchipelago;

use self::currency::{
    decimal_currency_system_press, non_decimal_currency_system_press, time_currency_system_press,
};

use super::common::general_currency_name_die;

/// # Arguments
/// * `decimal_nondecimal_time_based_choice`: &str. "" / _ = random, "decimal" / "time" / "nondecimal" for fine controll
pub fn currency_press(
    data: &UnitArchipelago,
    rng: &mut Fortuna,
    unicode: &Thoth,
    decimal_nondecimal_time_based_choice: &str,
) -> Result<String, Error> {
    let currency_name = general_currency_name_die(data, rng)?;
    match decimal_nondecimal_time_based_choice {
        "decimal" => decimal_currency_system_press(data, rng, unicode, &currency_name),
        "time" => time_currency_system_press(data, rng, unicode, &currency_name),
        "nondecimal" => non_decimal_currency_system_press(data, rng, unicode, &currency_name),
        _ => {
            if let Ok(n) = rng.random_from_range(0, 2) {
                match n {
                    0 => decimal_currency_system_press(data, rng, unicode, &currency_name),
                    1 => time_currency_system_press(data, rng, unicode, &currency_name),
                    _ => non_decimal_currency_system_press(data, rng, unicode, &currency_name),
                }
            } else {
                Err(Error::other("Failed to generate currency system"))
            }
        }
    }
}

mod currency {
    use std::io::Error;

    use fortuna::{Fortuna, RngApi};
    use thoth::Thoth;

    use crate::{
        minting::presses::{
            common::{
                artifact_type_type, formal_name_die, full_people_name_die,
                general_currency_name_die, random_abc_type,
            },
            metal_alloy_press::metal_alloy_press,
        },
        unit_pools::UnitArchipelago,
    };

    pub fn non_decimal_currency_system_press(
        data: &UnitArchipelago,
        rng: &mut Fortuna,
        unicode: &Thoth,
        currency_name: &str,
    ) -> Result<String, Error> {
        if let Ok(n) = rng.random_from_range(0, 5) {
            non_decimal_currency_system_die(data, rng, unicode, currency_name, n)
        } else {
            non_decimal_currency_system_die(data, rng, unicode, currency_name, 0)
        }
    }

    pub fn decimal_currency_system_press(
        data: &UnitArchipelago,
        rng: &mut Fortuna,
        unicode: &Thoth,
        currency_name: &str,
    ) -> Result<String, Error> {
        if let Ok(n) = rng.random_from_range(0, 5) {
            decimal_currency_system_die(data, rng, unicode, currency_name, n)
        } else {
            decimal_currency_system_die(data, rng, unicode, currency_name, 0)
        }
    }

    pub fn time_currency_system_press(
        data: &UnitArchipelago,
        rng: &mut Fortuna,
        unicode: &Thoth,
        currency_name: &str,
    ) -> Result<String, Error> {
        if let Ok(n) = rng.random_from_range(0, 5) {
            time_currency_system_die(data, rng, unicode, currency_name, n)
        } else {
            time_currency_system_die(data, rng, unicode, currency_name, 0)
        }
    }

    pub fn decimal_currency_system_die(
        data: &UnitArchipelago,
        rng: &mut Fortuna,
        unicode: &Thoth,
        currency_name: &str,
        unit_amount: usize,
    ) -> Result<String, Error> {
        let abbr = currency_abbreviation_type(rng, unicode, currency_name)?;
        let mut out: Vec<String> = vec![
            format!("This is a decimal currency."),
            format!("The currency is called {currency_name} and abbreviated as {abbr}."),
        ];
        // Main unit
        out.push(format!("The main unit is called {currency_name}"));
        out.push(currency_coin_denomination_and_material_die(
            data,
            rng,
            &currency_name,
        )?);
        // Rest unit generation
        if unit_amount > 0 {
            let mut last_unit_name: String = currency_name.to_string();
            let mut first_sub_unit_taken = false;
            for unit in 0..unit_amount {
                if unit == 0 {
                    // Super unit
                    // It's rare in real life so 1 in 10 should suffice.
                    if let Ok(n) = rng.random_from_range(0, 9)
                        && n == 0
                    {
                        let super_unit_name = general_currency_name_die(data, rng)?;
                        out.insert(2, format!("The super-unit is called {super_unit_name}."));
                        out.insert(
                            3,
                            currency_coin_denomination_and_material_die(
                                data,
                                rng,
                                &super_unit_name,
                            )?,
                        );
                        out.insert(
                            4,
                            format!(
                                "The super unit {} exchanges for 1 {} = 100 {}.",
                                super_unit_name, super_unit_name, currency_name
                            ),
                        );
                    } else {
                        // another sub unit instead, remember unit == 0!
                        let sub_unit_name = general_currency_name_die(data, rng)?;
                        out.push(format!("The 1. sub-unit is called {sub_unit_name}."));
                        out.push(currency_coin_denomination_and_material_die(
                            data,
                            rng,
                            &sub_unit_name,
                        )?);
                        out.push(format!("The 1. sub-unit {sub_unit_name} exchanges for 100 {sub_unit_name} = 1 {last_unit_name}"));
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
                    let sub_unit_name = general_currency_name_die(data, rng)?;
                    out.push(format!(
                        "The {actual_number}. sub-unit is called {sub_unit_name}."
                    ));
                    out.push(currency_coin_denomination_and_material_die(
                        data,
                        rng,
                        &sub_unit_name,
                    )?);
                    out.push(format!("The {actual_number}. sub-unit {sub_unit_name} exchanges for 100 {sub_unit_name} = 1 {last_unit_name}"));
                    last_unit_name = sub_unit_name;
                }
            }
        }
        Ok(out.join("\n"))
    }

    pub fn non_decimal_currency_system_die(
        data: &UnitArchipelago,
        rng: &mut Fortuna,
        unicode: &Thoth,
        currency_name: &str,
        unit_amount: usize,
    ) -> Result<String, Error> {
        let abbr = currency_abbreviation_type(rng, unicode, currency_name)?;
        let mut out: Vec<String> = vec![
            format!("This is a non decimal currency."),
            format!("The currency is called {currency_name} and abbreviated as {abbr}."),
        ];
        // Main unit
        out.push(format!("The main unit is called {currency_name}"));
        out.push(currency_coin_denomination_and_material_die(
            data,
            rng,
            &currency_name,
        )?);
        // Safety check!
        if unit_amount
            > data
                .numerical_unit_lagoon
                .currency_non_decimal_base
                .unit_pool
                .len()
        {
            return Err(Error::other(format!(
                "Only {} different coin values in database. Requested {} different coin values.",
                data.numerical_unit_lagoon
                    .currency_non_decimal_base
                    .unit_pool
                    .len(),
                unit_amount
            )));
        }
        // Rest unit generation
        if unit_amount > 0 {
            let mut bases = data
                .numerical_unit_lagoon
                .currency_non_decimal_base
                .unit_pool
                .clone();
            let mut last_unit_name: String = currency_name.to_string();
            let mut first_sub_unit_taken = false;
            for unit in 0..unit_amount {
                if unit == 0 {
                    // Super unit
                    // It's rare in real life so 1 in 10 should suffice.
                    if let Ok(n) = rng.random_from_range(0, 9)
                        && n == 0
                    {
                        let super_unit_name = general_currency_name_die(data, rng)?;
                        let idx = rng.random_index(bases.len()).unwrap_or(0);
                        let super_base = bases.remove(idx);
                        out.insert(2, format!("The super-unit is called {super_unit_name}."));
                        out.insert(
                            3,
                            currency_coin_denomination_and_material_die(
                                data,
                                rng,
                                &super_unit_name,
                            )?,
                        );
                        out.insert(
                            4,
                            format!(
                                "The super unit {} exchanges for 1 {} = {} {}.",
                                super_unit_name, super_unit_name, super_base, currency_name
                            ),
                        );
                    } else {
                        // another sub unit instead, remember unit == 0!
                        let sub_unit_name = general_currency_name_die(data, rng)?;
                        let idx = rng.random_index(bases.len()).unwrap_or(0);
                        let sub_base = bases.remove(idx);
                        out.push(format!("The 1. sub-unit is called {sub_unit_name}."));
                        out.push(currency_coin_denomination_and_material_die(
                            data,
                            rng,
                            &sub_unit_name,
                        )?);
                        out.push(format!("The 1. sub-unit {sub_unit_name} exchanges for {sub_base} {sub_unit_name} = 1 {last_unit_name}"));
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
                    let sub_unit_name = general_currency_name_die(data, rng)?;
                    let idx = rng.random_index(bases.len()).unwrap_or(0);
                    let sub_base = bases.remove(idx);
                    out.push(format!(
                        "The {actual_number}. sub-unit is called {sub_unit_name}."
                    ));
                    out.push(currency_coin_denomination_and_material_die(
                        data,
                        rng,
                        &sub_unit_name,
                    )?);
                    out.push(format!("The {actual_number}. sub-unit {sub_unit_name} exchanges for {sub_base} {sub_unit_name} = 1 {last_unit_name}"));
                    last_unit_name = sub_unit_name;
                }
            }
        }
        Ok(out.join("\n"))
    }

    pub fn time_currency_system_die(
        data: &UnitArchipelago,
        rng: &mut Fortuna,
        unicode: &Thoth,
        currency_name: &str,
        unit_amount: usize,
    ) -> Result<String, Error> {
        let abbr = currency_abbreviation_type(rng, unicode, currency_name)?;
        let mut out: Vec<String> = vec![
            format!("This is a time based currency."),
            format!("The currency is called {currency_name} and abbreviated as {abbr}."),
        ];
        // Main unit
        out.push(format!("The main unit is called {currency_name} and 1 {currency_name} is equal to {} hours of work for the good of society.", currency_time_fractions_type(&data, rng)?));
        out.push(currency_coin_denomination_and_material_die(
            data,
            rng,
            &currency_name,
        )?);
        // Rest unit generation
        if unit_amount > 0 {
            let mut bases = data
                .numerical_unit_lagoon
                .currency_time_fractions
                .unit_pool
                .clone();
            let _ = bases.remove(0);
            let mut last_unit_name: String = currency_name.to_string();
            let mut first_sub_unit_taken = false;
            for unit in 0..unit_amount {
                if unit == 0 {
                    // Super unit
                    // It's rare in real life so 1 in 10 should suffice.
                    if let Ok(n) = rng.random_from_range(0, 9)
                        && n == 0
                    {
                        let super_unit_name = general_currency_name_die(data, rng)?;
                        let idx = rng.random_index(bases.len()).unwrap_or(0);
                        let super_base = bases.remove(idx);
                        out.insert(2, format!("The super-unit is called {super_unit_name}."));
                        out.insert(
                            3,
                            currency_coin_denomination_and_material_die(
                                data,
                                rng,
                                &super_unit_name,
                            )?,
                        );
                        out.insert(
                            4,
                            format!(
                                "The super unit {} exchanges for 1 {} = {} {}.",
                                super_unit_name, super_unit_name, super_base, currency_name
                            ),
                        );
                    } else {
                        // another sub unit instead, remember unit == 0!
                        let sub_unit_name = general_currency_name_die(data, rng)?;
                        let idx = rng.random_index(bases.len()).unwrap_or(0);
                        let sub_base = bases.remove(idx);
                        out.push(format!("The 1. sub-unit is called {sub_unit_name}."));
                        out.push(currency_coin_denomination_and_material_die(
                            data,
                            rng,
                            &sub_unit_name,
                        )?);
                        out.push(format!("The 1. sub-unit {sub_unit_name} exchanges for {sub_base} {sub_unit_name} = 1 {last_unit_name}"));
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
                    let sub_unit_name = general_currency_name_die(data, rng)?;
                    let idx = rng.random_index(bases.len()).unwrap_or(0);
                    let sub_base = bases.remove(idx);
                    out.push(format!(
                        "The {actual_number}. sub-unit is called {sub_unit_name}."
                    ));
                    out.push(currency_coin_denomination_and_material_die(
                        data,
                        rng,
                        &sub_unit_name,
                    )?);
                    out.push(format!("The {actual_number}. sub-unit {sub_unit_name} exchanges for {sub_base} {sub_unit_name} = 1 {last_unit_name}"));
                    last_unit_name = sub_unit_name;
                }
            }
        }
        Ok(out.join("\n"))
    }

    pub fn currency_coin_denomination_and_material_die(
        data: &UnitArchipelago,
        rng: &mut Fortuna,
        unit_name: &str,
    ) -> Result<String, Error> {
        let coin_num = {
            if let Ok(n) = rng.random_from_range(1, 7) {
                n
            } else {
                return Err(Error::other(
                    "currency_coin_denomination_and_material_die failed",
                ));
            }
        };
        let mut denominations: Vec<String> = Default::default();
        for _ in 0..coin_num {
            denominations.push(currency_coin_denomination_type(data, rng)?.to_string());
        }
        let mut out: Vec<String> = vec![format!(
            "The {} has the 1 coin. The coin {}. {}.",
            unit_name,
            currency_material_type(data, rng)?,
            currency_coin_iconography_press(data, rng)?
        )];
        for denomination in denominations {
            out.push(format!(
                "The {denomination} coin, {}. {}.",
                currency_material_type(data, rng)?,
                currency_coin_iconography_press(data, rng)?
            ));
        }
        Ok(out.join("\n"))
    }

    pub fn currency_coin_iconography_press(
        data: &UnitArchipelago,
        rng: &mut Fortuna,
    ) -> Result<String, Error> {
        let out = [
            format!(
                "It shows {} on it's front,",
                currency_coin_iconography_die(data, rng, false)?
            ),
            format!(
                "and it shows {} on it's back",
                currency_coin_iconography_die(data, rng, true)?
            ),
        ];
        Ok(out.join("\n"))
    }

    pub fn currency_coin_iconography_die(
        data: &UnitArchipelago,
        rng: &mut Fortuna,
        reverse: bool,
    ) -> Result<String, Error> {
        let seed = { if reverse { 4 } else { 3 } };
        if let Ok(n) = rng.random_from_range(0, seed) {
            match n {
                0 => currency_icon_inscription_type(&data, rng),
                1 => currency_icon_figure_type(&data, rng),
                2 => currency_icon_partial_die(&data, rng),
                3 => Ok(format!(
                    "a incuse of a {}",
                    currency_icon_incuse_type(&data, rng)?
                )),
                _ => Ok("the same motif repeated".to_string()),
            }
        } else {
            Err(Error::other("currency_coin_iconography_die failed"))
        }
    }

    pub fn currency_icon_figure_type(
        data: &UnitArchipelago,
        rng: &mut Fortuna,
    ) -> Result<String, Error> {
        let type_to_check = currency_figure_type(&data, rng)?;
        let rand_range = if let Ok(range) = rng.random_from_range(0, 4) {
            range
        } else {
            return Err(Error::other("currency_icon_figure_type failed"));
        };
        match type_to_check {
            "Person" => {
                let gender_to_check = currency_person_social_gender_type(&data, rng)?;
                match gender_to_check {
                    "male" => Ok(format!(
                        "a male figure with {}",
                        currency_coin_bonus_iconography_type(&data, rng)?
                    )),
                    "female" => Ok(format!(
                        "a female figure with {}",
                        currency_coin_bonus_iconography_type(&data, rng)?
                    )),
                    "indiscernible" => Ok(format!(
                        "a indiscernible figure with {}",
                        currency_coin_bonus_iconography_type(&data, rng)?
                    )),
                    _ => Err(Error::other(format!(
                        "{gender_to_check} from json library is not implemented in icon_figure_type!"
                    ))),
                }
            }
            "Creature" => match rand_range {
                0 => Ok(format!(
                    "a mythical {} and a {}",
                    currency_icon_mythical_creature_type(&data, rng)?,
                    currency_coin_bonus_iconography_type(&data, rng)?
                )),
                _ => Ok(format!(
                    "a mythical {}",
                    currency_icon_mythical_creature_type(&data, rng)?
                )),
            },
            "Animal" => match rand_range {
                0 => Ok(format!(
                    "a {} and a {}",
                    currency_icon_animal_type(&data, rng)?,
                    currency_coin_bonus_iconography_type(&data, rng)?
                )),
                _ => Ok(format!("a {}", currency_icon_animal_type(&data, rng)?)),
            },
            "Plant" => match rand_range {
                0 => Ok(format!(
                    "a {} and a {}",
                    currency_icon_plant_type(&data, rng)?,
                    currency_coin_bonus_iconography_type(&data, rng)?
                )),
                _ => Ok(format!("a {}", currency_icon_plant_type(&data, rng)?)),
            },
            "Object" => match rand_range {
                0 => Ok(format!("a {}", currency_figure_type(&data, rng)?)),
                _ => Ok(format!("a {}", artifact_type_type(&data, rng)?)),
            },
            _ => Err(Error::other(format!(
                "{type_to_check} from json library is not implemented in icon_figure_type!"
            ))),
        }
    }

    pub fn currency_icon_partial_die(
        data: &UnitArchipelago,
        rng: &mut Fortuna,
    ) -> Result<String, Error> {
        if let Ok(n) = rng.random_from_range(0, 4) {
            match n {
                0 => Ok(format!(
                    "a {} and a {}",
                    &currency_icon_partial_type(data, rng)?,
                    &currency_coin_bonus_iconography_type(data, rng)?
                )),
                _ => Ok(format!("a {}", currency_icon_partial_type(data, rng)?)),
            }
        } else {
            Err(Error::other("currency_icon_partial_die failed"))
        }
    }

    pub fn currency_coin_bonus_iconography_type(
        data: &UnitArchipelago,
        rng: &mut Fortuna,
    ) -> Result<String, Error> {
        let insignia = {
            if let Ok(n) = rng.random_from_range(0, 1)
                && n == 0
            {
                &artifact_type_type(data, rng)?
            } else {
                currency_icon_insignia_type(data, rng)?
            }
        };
        Ok(format!("a {insignia}"))
    }

    pub fn currency_icon_inscription_type(
        data: &UnitArchipelago,
        rng: &mut Fortuna,
    ) -> Result<String, Error> {
        let type_to_check = currency_inscription_type(data, rng)?;
        match type_to_check {
            "Letter" => Ok(format!(
                "the letter {}",
                random_abc_type(data, rng)?.to_uppercase()
            )),
            "Monogram" => Ok(format!(
                "a monogram of the letters {} and {}",
                random_abc_type(data, rng)?.to_uppercase(),
                random_abc_type(data, rng)?.to_uppercase()
            )),
            "Name" => {
                if let Ok(n) = rng.random_from_range(0, 1)
                    && n == 0
                {
                    formal_name_die(data, rng)
                } else {
                    full_people_name_die(data, rng)
                }
            }
            _ => Err(Error::other(format!(
                "{type_to_check} from json library is not implemented in icon_inscription_type!"
            ))),
        }
    }

    pub fn currency_material_type(
        data: &UnitArchipelago,
        rng: &mut Fortuna,
    ) -> Result<String, Error> {
        if let Ok(n) = rng.random_from_range(0, 1)
            && n == 0
        {
            // mono metallic coin
            Ok(format!("is made of {}", metal_alloy_press(data, rng)?))
        } else {
            // bi metallic coin
            Ok(format!(
                "is made with a {} center plug and a {} outer ring",
                metal_alloy_press(data, rng)?,
                metal_alloy_press(data, rng)?
            ))
        }
    }

    pub fn currency_abbreviation_type(
        rng: &mut Fortuna,
        unicode: &Thoth,
        currency_name: &str,
    ) -> Result<String, Error> {
        if let Ok(n) = rng.random_from_range(0, 1)
            && n == 0
        {
            if currency_name.len() >= 3 {
                let tmp_name_bind = {
                    if let Ok(inner) = unicode.segment(currency_name) {
                        inner
                    } else {
                        return Err(Error::other(format!("Unicode segmentation failed!")));
                    }
                };
                let first_letter = tmp_name_bind.get(0).expect("Len is at least 3");
                let (idx0, idx1) = {
                    if let Ok(idx0) = rng.random_index(tmp_name_bind.len())
                        && let Ok(idx1) = rng.random_index(tmp_name_bind.len())
                    {
                        (idx0, idx1)
                    } else {
                        return Err(Error::other(format!("Random index failed!")));
                    }
                };
                let letter1 = tmp_name_bind
                    .get(idx0)
                    .expect("Idx is guaranteed to be valid");
                let letter2 = tmp_name_bind
                    .get(idx1)
                    .expect("Idx is guaranteed to be valid");
                Ok(format!(
                    "{}{}{}",
                    first_letter.to_uppercase(),
                    letter1,
                    letter2
                ))
            } else {
                Err(Error::other(format!(
                    "Name ({currency_name}) is shorter than 3 letters!"
                )))
            }
        } else {
            if currency_name.len() >= 3 {
                let tmp_name_bind = {
                    if let Ok(inner) = unicode.segment(currency_name) {
                        inner
                    } else {
                        return Err(Error::other(format!("Unicode segmentation failed!")));
                    }
                };
                let first_letter = tmp_name_bind.get(0).expect("Len is at least 3");
                let letter1 = tmp_name_bind.get(1).expect("Len is at least 3");
                let letter2 = tmp_name_bind.get(2).expect("Len is at least 3");
                Ok(format!(
                    "{}{}{}",
                    first_letter.to_uppercase(),
                    letter1,
                    letter2
                ))
            } else {
                Err(Error::other(format!(
                    "Name ({currency_name}) is shorter than 3 letters!"
                )))
            }
        }
    }

    pub fn currency_time_fractions_type<'a>(
        data: &'a UnitArchipelago,
        rng: &mut Fortuna,
    ) -> Result<&'a f32, Error> {
        if let Ok(idx) = rng.random_index(
            data.numerical_unit_lagoon
                .currency_time_fractions
                .unit_pool
                .len(),
        ) {
            Ok(&data.numerical_unit_lagoon.currency_time_fractions.unit_pool[idx])
        } else {
            Err(Error::other("currency_time_fractions_type failed"))
        }
    }

    pub fn currency_coin_denomination_type<'a>(
        data: &'a UnitArchipelago,
        rng: &mut Fortuna,
    ) -> Result<&'a f32, Error> {
        if let Ok(idx) = rng.random_index(
            data.numerical_unit_lagoon
                .currency_coins_denomination
                .unit_pool
                .len(),
        ) {
            Ok(&data
                .numerical_unit_lagoon
                .currency_coins_denomination
                .unit_pool[idx])
        } else {
            Err(Error::other("currency_coin_denomination_type failed"))
        }
    }

    pub fn currency_inscription_type<'a>(
        data: &'a UnitArchipelago,
        rng: &mut Fortuna,
    ) -> Result<&'a str, Error> {
        if let Ok(idx) = rng.random_index(
            data.lexical_unit_lagoon
                .currency_icon_inscription
                .unit_pool
                .len(),
        ) {
            Ok(&data.lexical_unit_lagoon.currency_icon_inscription.unit_pool[idx])
        } else {
            Err(Error::other("currency_inscription_type failed"))
        }
    }

    pub fn currency_figure_type<'a>(
        data: &'a UnitArchipelago,
        rng: &mut Fortuna,
    ) -> Result<&'a str, Error> {
        if let Ok(idx) = rng.random_index(
            data.lexical_unit_lagoon
                .currency_icon_figure
                .unit_pool
                .len(),
        ) {
            Ok(&data.lexical_unit_lagoon.currency_icon_figure.unit_pool[idx])
        } else {
            Err(Error::other("currency_figure_type failed"))
        }
    }

    pub fn currency_person_social_gender_type<'a>(
        data: &'a UnitArchipelago,
        rng: &mut Fortuna,
    ) -> Result<&'a str, Error> {
        if let Ok(idx) = rng.random_index(
            data.lexical_unit_lagoon
                .person_social_gender
                .unit_pool
                .len(),
        ) {
            Ok(&data.lexical_unit_lagoon.person_social_gender.unit_pool[idx])
        } else {
            Err(Error::other("currency_person_social_gender_type failed"))
        }
    }

    pub fn currency_icon_insignia_type<'a>(
        data: &'a UnitArchipelago,
        rng: &mut Fortuna,
    ) -> Result<&'a str, Error> {
        if let Ok(idx) = rng.random_index(
            data.lexical_unit_lagoon
                .currency_icon_insignia
                .unit_pool
                .len(),
        ) {
            Ok(&data.lexical_unit_lagoon.currency_icon_insignia.unit_pool[idx])
        } else {
            Err(Error::other("currency_icon_insignia_type failed"))
        }
    }

    pub fn currency_icon_mythical_creature_type<'a>(
        data: &'a UnitArchipelago,
        rng: &mut Fortuna,
    ) -> Result<&'a str, Error> {
        if let Ok(idx) = rng.random_index(
            data.lexical_unit_lagoon
                .currency_icon_mythical_creature
                .unit_pool
                .len(),
        ) {
            Ok(&data
                .lexical_unit_lagoon
                .currency_icon_mythical_creature
                .unit_pool[idx])
        } else {
            Err(Error::other("currency_icon_mythical_creature_type failed"))
        }
    }

    pub fn currency_icon_animal_type<'a>(
        data: &'a UnitArchipelago,
        rng: &mut Fortuna,
    ) -> Result<&'a str, Error> {
        if let Ok(idx) = rng.random_index(
            data.lexical_unit_lagoon
                .currency_icon_animal
                .unit_pool
                .len(),
        ) {
            Ok(&data.lexical_unit_lagoon.currency_icon_animal.unit_pool[idx])
        } else {
            Err(Error::other("currency_icon_animal_type failed"))
        }
    }

    pub fn currency_icon_plant_type<'a>(
        data: &'a UnitArchipelago,
        rng: &mut Fortuna,
    ) -> Result<&'a str, Error> {
        if let Ok(idx) =
            rng.random_index(data.lexical_unit_lagoon.currency_icon_plant.unit_pool.len())
        {
            Ok(&data.lexical_unit_lagoon.currency_icon_plant.unit_pool[idx])
        } else {
            Err(Error::other("currency_icon_plant_type failed"))
        }
    }

    pub fn currency_icon_partial_type<'a>(
        data: &'a UnitArchipelago,
        rng: &mut Fortuna,
    ) -> Result<&'a str, Error> {
        if let Ok(idx) = rng.random_index(
            data.lexical_unit_lagoon
                .currency_icon_partial
                .unit_pool
                .len(),
        ) {
            Ok(&data.lexical_unit_lagoon.currency_icon_partial.unit_pool[idx])
        } else {
            Err(Error::other("currency_icon_partial_type failed"))
        }
    }

    pub fn currency_icon_incuse_type<'a>(
        data: &'a UnitArchipelago,
        rng: &mut Fortuna,
    ) -> Result<&'a str, Error> {
        if let Ok(idx) = rng.random_index(
            data.lexical_unit_lagoon
                .currency_icon_incuse
                .unit_pool
                .len(),
        ) {
            Ok(&data.lexical_unit_lagoon.currency_icon_incuse.unit_pool[idx])
        } else {
            Err(Error::other("currency_icon_incuse_type failed"))
        }
    }
}
