use std::{io::{Error, ErrorKind}, path::Path};

use crate::jisard::read_json;

use super::{numerical_unit_pool::NumericalUnitPool, numerical_unit_dictionary::NumericalUnitDictionary};


#[derive(Debug, Clone, Default)]
pub struct NumericalUnitLagoon {
    pub currency_time_fractions: NumericalUnitPool,
    pub currency_non_decimal_base: NumericalUnitPool,
    pub currency_coins_denomination: NumericalUnitPool,
    pub story_town_trade_goods_dict: NumericalUnitDictionary,
}

impl NumericalUnitLagoon {
    pub fn new(num_lib_path: &Path) -> Result<NumericalUnitLagoon, Error> {
        let mut story_town_trade_goods_dict: NumericalUnitDictionary = Default::default();

        let mut currency_time_fractions: NumericalUnitPool = Default::default();
        let mut currency_non_decimal_base: NumericalUnitPool = Default::default();
        let mut currency_coins_denomination: NumericalUnitPool = Default::default();

        match read_json(num_lib_path) {
            Ok(data) => {
                for entry in data.entries() {
                    match entry.0 {
                            "story_town_trade_goods_dict" => {
                            story_town_trade_goods_dict = NumericalUnitDictionary::from(entry);
                        },
                            "currency_time_fractions" => {
                            currency_time_fractions = NumericalUnitPool::from(entry);
                        },
                            "currency_non_decimal_base_120" => {
                            currency_non_decimal_base = NumericalUnitPool::from(entry);
                        },
                            "currency_coins_denomination" => {
                            currency_coins_denomination = NumericalUnitPool::from(entry);
                        },
                        _ => {
                            Err(Error::other(format!("Undeclared json list. {} (in {:?}) is not implemented!", entry.0, num_lib_path )))?
                        },
                    };
                }
                Ok(NumericalUnitLagoon { currency_time_fractions, story_town_trade_goods_dict, currency_non_decimal_base, currency_coins_denomination })
            },
            Err(error) => {
                Err(Error::new(ErrorKind::Other, error))
            },
        }
    }
}
