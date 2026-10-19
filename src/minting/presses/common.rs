use std::io::Error;

use fortuna::{Fortuna, RngApi};

use crate::{minting::minting_type::SkillLevel, unit_pools::UnitArchipelago};

pub fn comp_name_type(data: &UnitArchipelago, rng: &mut Fortuna) -> Result<String, Error> {
    if let Ok(comp0_idx) = rng.random_index(data.lexical_unit_lagoon.general_comp0.unit_pool.len())
        && let Ok(comp1_idx) =
            rng.random_index(data.lexical_unit_lagoon.general_comp1.unit_pool.len())
    {
        Ok(format!(
            "{}{}",
            data.lexical_unit_lagoon.general_comp0.unit_pool[comp0_idx],
            data.lexical_unit_lagoon.general_comp1.unit_pool[comp1_idx],
        ))
    } else {
        Err(Error::other("comp_name_type failed"))
    }
}

pub fn formal_name_die(data: &UnitArchipelago, rng: &mut Fortuna) -> Result<String, Error> {
    if let Ok(idx) = rng.random_index(data.lexical_unit_lagoon.people_title.unit_pool.len()) {
        Ok(format!(
            "{} {}",
            data.clone().lexical_unit_lagoon.people_title.unit_pool[idx],
            legal_name_die(data, rng)?
        ))
    } else {
        Err(Error::other("formal_name_die failed"))
    }
}

pub fn formal_last_name_die(data: &UnitArchipelago, rng: &mut Fortuna) -> Result<String, Error> {
    if let Ok(idx) = rng.random_index(data.lexical_unit_lagoon.people_title.unit_pool.len()) {
        Ok(format!(
            "{} {}",
            data.clone().lexical_unit_lagoon.people_title.unit_pool[idx],
            last_name_type(data, rng)?
        ))
    } else {
        Err(Error::other("formal_last_name_die failed"))
    }
}

pub fn full_people_name_die(data: &UnitArchipelago, rng: &mut Fortuna) -> Result<String, Error> {
    Ok(format!(
        "Name: {}. Nickname: {}",
        legal_name_die(data, rng)?,
        nickname_type(data, rng)?
    ))
}

pub fn legal_name_die(data: &UnitArchipelago, rng: &mut Fortuna) -> Result<String, Error> {
    Ok(format!(
        "{} {}",
        first_name_type(data, rng)?,
        last_name_type(data, rng)?
    ))
}

pub fn simple_name_type(data: &UnitArchipelago, rng: &mut Fortuna) -> Result<String, Error> {
    if let Ok(seed) = rng.random_from_range(0, 2) {
        if seed >= 2
            && let Ok(idx) = rng.random_index(data.lexical_unit_lagoon.place_single.unit_pool.len())
        {
            Ok(data.lexical_unit_lagoon.place_single.unit_pool[idx].clone())
        } else if seed == 1
            && let Ok(idx) =
                rng.random_index(data.lexical_unit_lagoon.people_nickname.unit_pool.len())
        {
            Ok(data.lexical_unit_lagoon.people_nickname.unit_pool[idx].clone())
        } else {
            comp_name_type(data, rng)
        }
    } else {
        comp_name_type(data, rng)
    }
}

pub fn empire_name_type(data: &UnitArchipelago, rng: &mut Fortuna) -> Result<String, Error> {
    if let Ok(n) = rng.random_from_range(0, 1)
        && n == 1
    {
        simple_name_type(data, rng)
    } else {
        comp_name_type(data, rng)
    }
}

pub fn full_place_name_die(data: &UnitArchipelago, rng: &mut Fortuna) -> Result<String, Error> {
    if let Ok(n) = rng.random_from_range(0, 3) {
        match n {
            3 => simple_name_type(data, rng),
            2 => Ok(format!(
                "{} {}",
                simple_name_type(data, rng)?,
                simple_name_type(data, rng)?
            )),
            1 => Ok(format!(
                "{}'s {}",
                comp_name_type(data, rng)?,
                simple_name_type(data, rng)?
            )),
            _ => comp_name_type(data, rng),
        }
    } else {
        Err(Error::other("full_place_name_die failed"))
    }
}

pub fn nickname_type(data: &UnitArchipelago, rng: &mut Fortuna) -> Result<String, Error> {
    if let Ok(n) = rng.random_from_range(0, 2)
        && n == 0
        && let Ok(idx) = rng.random_index(data.lexical_unit_lagoon.people_nickname.unit_pool.len())
    {
        Ok(data.lexical_unit_lagoon.people_nickname.unit_pool[idx].clone())
    } else {
        Ok(String::default())
    }
}

pub fn first_name_type(data: &UnitArchipelago, rng: &mut Fortuna) -> Result<String, Error> {
    if let Ok(n) = rng.random_from_range(0, 1)
        && n == 0
    {
        comp_name_type(data, rng)
    } else if let Ok(idx) =
        rng.random_index(data.lexical_unit_lagoon.people_first_name.unit_pool.len())
    {
        Ok(data.lexical_unit_lagoon.people_first_name.unit_pool[idx].clone())
    } else {
        Err(Error::other("first_name_type failed"))
    }
}

pub fn last_name_type(data: &UnitArchipelago, rng: &mut Fortuna) -> Result<String, Error> {
    if let Ok(n) = rng.random_from_range(0, 1)
        && n == 0
    {
        comp_name_type(data, rng)
    } else if let Ok(idx) =
        rng.random_index(data.lexical_unit_lagoon.people_last_name.unit_pool.len())
    {
        Ok(data.lexical_unit_lagoon.people_last_name.unit_pool[idx].clone())
    } else {
        Err(Error::other("last_name_type failed"))
    }
}

pub fn level_capped_skill_die(
    data: &UnitArchipelago,
    rng: &mut Fortuna,
    skill_level: SkillLevel,
) -> Result<(String, String), Error> {
    let level = match skill_level {
        SkillLevel::Any => skill_die(data, rng)?,
        SkillLevel::Low => low_skill_die(data, rng)?.to_string(),
        SkillLevel::Medium => medium_skill_die(data, rng)?.to_string(),
        SkillLevel::High => high_skill_die(data, rng)?.to_string(),
        SkillLevel::Ultra => ultra_skill_die(data, rng)?.to_string(),
    };
    let skill =
        if let Ok(idx) = rng.random_index(data.lexical_unit_lagoon.people_skill.unit_pool.len()) {
            data.lexical_unit_lagoon.people_skill.unit_pool[idx].clone()
        } else {
            return Err(Error::other(
                "level_capped_skill_die failed; Cannot get skill kind",
            ));
        };
    Ok((level, skill))
}

fn low_skill_die<'a>(data: &'a UnitArchipelago, rng: &mut Fortuna) -> Result<&'a str, Error> {
    if let Ok(idx) = rng.random_index(
        data.lexical_unit_lagoon
            .people_skill_level_low
            .unit_pool
            .len(),
    ) {
        Ok(&data.lexical_unit_lagoon.people_skill_level_low.unit_pool[idx])
    } else {
        Err(Error::other("low_skill_die failed"))
    }
}
fn medium_skill_die<'a>(data: &'a UnitArchipelago, rng: &mut Fortuna) -> Result<&'a str, Error> {
    if let Ok(idx) = rng.random_index(
        data.lexical_unit_lagoon
            .people_skill_level_medium
            .unit_pool
            .len(),
    ) {
        Ok(&data.lexical_unit_lagoon.people_skill_level_medium.unit_pool[idx])
    } else {
        Err(Error::other("medium_skill_die failed"))
    }
}
fn high_skill_die<'a>(data: &'a UnitArchipelago, rng: &mut Fortuna) -> Result<&'a str, Error> {
    if let Ok(idx) = rng.random_index(
        data.lexical_unit_lagoon
            .people_skill_level_high
            .unit_pool
            .len(),
    ) {
        Ok(&data.lexical_unit_lagoon.people_skill_level_high.unit_pool[idx])
    } else {
        Err(Error::other("high_skill_die failed"))
    }
}
fn ultra_skill_die<'a>(data: &'a UnitArchipelago, rng: &mut Fortuna) -> Result<&'a str, Error> {
    if let Ok(idx) = rng.random_index(
        data.lexical_unit_lagoon
            .people_skill_level_ultra
            .unit_pool
            .len(),
    ) {
        Ok(&data.lexical_unit_lagoon.people_skill_level_ultra.unit_pool[idx])
    } else {
        Err(Error::other("ultra_skill_die failed"))
    }
}

pub fn skill_die(data: &UnitArchipelago, rng: &mut Fortuna) -> Result<String, Error> {
    if let Ok(idx_skill_level) = rng.random_index(
        data.lexical_unit_lagoon
            .people_skill_level_all
            .unit_pool
            .len(),
    ) && let Ok(idx_skill) =
        rng.random_index(data.lexical_unit_lagoon.people_skill.unit_pool.len())
    {
        Ok(format!(
            "{} {}",
            data.lexical_unit_lagoon.people_skill_level_all.unit_pool[idx_skill_level],
            data.lexical_unit_lagoon.people_skill.unit_pool[idx_skill]
        ))
    } else {
        Err(Error::other("skill_die failed"))
    }
}

pub fn material_type<'a>(data: &'a UnitArchipelago, rng: &mut Fortuna) -> Result<&'a str, Error> {
    if let Ok(idx) = rng.random_index(data.lexical_unit_lagoon.artifact_material.unit_pool.len()) {
        Ok(&data.lexical_unit_lagoon.artifact_material.unit_pool[idx])
    } else {
        Err(Error::other("material_type failed"))
    }
}

pub fn artifact_type_type<'a>(
    data: &'a UnitArchipelago,
    rng: &mut Fortuna,
) -> Result<&'a str, Error> {
    if let Ok(idx) = rng.random_index(data.lexical_unit_lagoon.artifact_type.unit_pool.len()) {
        Ok(&data.lexical_unit_lagoon.artifact_type.unit_pool[idx])
    } else {
        Err(Error::other("artifact_type_type failed"))
    }
}

pub fn artifact_adjective_type<'a>(
    data: &'a UnitArchipelago,
    rng: &mut Fortuna,
) -> Result<&'a str, Error> {
    if let Ok(idx) = rng.random_index(data.lexical_unit_lagoon.artifact_adjective.unit_pool.len()) {
        Ok(&data.lexical_unit_lagoon.artifact_adjective.unit_pool[idx])
    } else {
        Err(Error::other("artifact_adjective_type failed"))
    }
}

pub fn quality_type<'a>(data: &'a UnitArchipelago, rng: &mut Fortuna) -> Result<&'a str, Error> {
    if let Ok(idx) = rng.random_index(data.lexical_unit_lagoon.artifact_quality.unit_pool.len()) {
        Ok(&data.lexical_unit_lagoon.artifact_quality.unit_pool[idx])
    } else {
        Err(Error::other("quality_type failed"))
    }
}

pub fn language_name_die(data: &UnitArchipelago, rng: &mut Fortuna) -> Result<String, Error> {
    if let Ok(n) = rng.random_from_range(0, 1)
        && n == 0
    {
        simple_name_type(data, rng)
    } else {
        comp_name_type(data, rng)
    }
}

pub fn general_currency_name_die(
    data: &UnitArchipelago,
    rng: &mut Fortuna,
) -> Result<String, Error> {
    if let Ok(n) = rng.random_from_range(0, 2) {
        match n {
            0 => {
                if let Ok(i) = rng.random_from_range(0, 2) {
                    match i {
                        0 => random_currency_name_type(data, rng, 3),
                        1 => random_currency_name_type(data, rng, 4),
                        _ => random_currency_name_type(data, rng, 5),
                    }
                } else {
                    Err(Error::other("general_currency_name_die failed"))
                }
            }
            _ => {
                if let Ok(n) = rng.random_from_range(0, 1)
                    && n == 0
                {
                    random_currency_real_fractional_name_type(data, rng).map(|x| x.to_string())
                } else {
                    random_currency_real_name_type(data, rng).map(|x| x.to_string())
                }
            }
        }
    } else {
        Err(Error::other("general_currency_name_die failed"))
    }
}

pub fn random_currency_name_type(
    data: &UnitArchipelago,
    rng: &mut Fortuna,
    random_letter_amount: usize,
) -> Result<String, Error> {
    if random_letter_amount == 0 {
        Err(Error::other(format!(
            "random_letter_amount may not be 0 (random_currency_name_type)"
        )))
    } else {
        let mut out: String = Default::default();
        for n in 0..random_letter_amount {
            if n == 0 {
                out.push_str(&random_abc_type(data, rng)?.to_uppercase());
            } else {
                out.push_str(&random_abc_type(data, rng)?);
            }
        }
        out.push_str(&currency_name_ending_die(data, rng)?);
        Ok(out)
    }
}

pub fn currency_name_ending_die<'a>(
    data: &'a UnitArchipelago,
    rng: &mut Fortuna,
) -> Result<&'a str, Error> {
    if let Ok(n) = rng.random_from_range(0, 1)
        && n == 0
    {
        random_currency_ending_type(data, rng)
    } else {
        random_currency_second_word_type(data, rng)
    }
}

pub fn random_abc_type<'a>(data: &'a UnitArchipelago, rng: &mut Fortuna) -> Result<&'a str, Error> {
    if let Ok(idx) = rng.random_index(data.lexical_unit_lagoon.abc.unit_pool.len()) {
        Ok(&data.lexical_unit_lagoon.abc.unit_pool[idx])
    } else {
        Err(Error::other("random_abc_type failed"))
    }
}

fn random_currency_ending_type<'a>(
    data: &'a UnitArchipelago,
    rng: &mut Fortuna,
) -> Result<&'a str, Error> {
    if let Ok(idx) = rng.random_index(data.lexical_unit_lagoon.currency_endings.unit_pool.len()) {
        Ok(&data.lexical_unit_lagoon.currency_endings.unit_pool[idx])
    } else {
        Err(Error::other("random_currency_ending_type failed"))
    }
}

fn random_currency_second_word_type<'a>(
    data: &'a UnitArchipelago,
    rng: &mut Fortuna,
) -> Result<&'a str, Error> {
    if let Ok(idx) = rng.random_index(
        data.lexical_unit_lagoon
            .currency_second_word
            .unit_pool
            .len(),
    ) {
        Ok(&data.lexical_unit_lagoon.currency_second_word.unit_pool[idx])
    } else {
        Err(Error::other("random_currency_second_word_type failed"))
    }
}

fn random_currency_real_fractional_name_type<'a>(
    data: &'a UnitArchipelago,
    rng: &mut Fortuna,
) -> Result<&'a str, Error> {
    if let Ok(idx) = rng.random_index(
        data.lexical_unit_lagoon
            .currency_real_fractional_names
            .unit_pool
            .len(),
    ) {
        Ok(&data
            .lexical_unit_lagoon
            .currency_real_fractional_names
            .unit_pool[idx])
    } else {
        Err(Error::other(
            "random_currency_real_fractional_name_type failed",
        ))
    }
}

fn random_currency_real_name_type<'a>(
    data: &'a UnitArchipelago,
    rng: &mut Fortuna,
) -> Result<&'a str, Error> {
    if let Ok(idx) = rng.random_index(data.lexical_unit_lagoon.currency_real_names.unit_pool.len())
    {
        Ok(&data.lexical_unit_lagoon.currency_real_names.unit_pool[idx])
    } else {
        Err(Error::other("random_currency_real_name_type failed"))
    }
}

pub fn random_place_noun_type<'a>(
    data: &'a UnitArchipelago,
    rng: &mut Fortuna,
) -> Result<&'a str, Error> {
    if let Ok(idx) = rng.random_index(data.lexical_unit_lagoon.place_noun.unit_pool.len()) {
        Ok(&data.lexical_unit_lagoon.place_noun.unit_pool[idx])
    } else {
        Err(Error::other("random_place_noun_type failed"))
    }
}
