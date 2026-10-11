use std::{io::Error, path::Path};

use crate::jisard::read_json;

use super::{
    numerical_unit_dictionary::NumericalUnitDictionary, numerical_unit_pool::NumericalUnitPool,
};

#[derive(Debug, Clone, Default)]
pub struct NumericalUnitLagoon {
    pub currency_time_fractions: NumericalUnitPool,
    pub currency_non_decimal_base: NumericalUnitPool,
    pub currency_coins_denomination: NumericalUnitPool,
    pub story_town_trade_goods_dict: NumericalUnitDictionary,
}

impl NumericalUnitLagoon {
    pub fn new(num_lib_path: &Path) -> Result<NumericalUnitLagoon, Error> {
        let story_town_trade_goods_dict: NumericalUnitDictionary;
        let currency_time_fractions: NumericalUnitPool;
        let currency_non_decimal_base: NumericalUnitPool;
        let currency_coins_denomination: NumericalUnitPool;

        match read_json(num_lib_path) {
            Ok(data) => {
                let obj = data.as_object().expect("JSON parent must be an object");
                story_town_trade_goods_dict = NumericalUnitDictionary::from((
                    "story_town_trade_goods_dict",
                    obj.get("story_town_trade_goods_dict")
                        .expect("Object must include key 'story_town_trade_goods_dict'"),
                ));
                currency_time_fractions = NumericalUnitPool::from((
                    "currency_time_fractions",
                    obj.get("currency_time_fractions")
                        .expect("Object must include key 'currency_time_fractions'"),
                ));
                currency_non_decimal_base = NumericalUnitPool::from((
                    "currency_non_decimal_base_120",
                    obj.get("currency_non_decimal_base_120")
                        .expect("Object must include key 'currency_non_decimal_base_120'"),
                ));
                currency_coins_denomination = NumericalUnitPool::from((
                    "currency_coins_denomination",
                    obj.get("currency_coins_denomination")
                        .expect("Object must include key 'currency_coins_denomination'"),
                ));
            }
            Err(error) => return Err(Error::other(error)),
        }
        Ok(NumericalUnitLagoon {
            currency_time_fractions,
            currency_non_decimal_base,
            currency_coins_denomination,
            story_town_trade_goods_dict,
        })
    }
}
