use super::{numerical_unit_pool::NumericalUnitPool, numerical_unit_dictionary::NumericalUnitDictionary};


#[derive(Debug, Clone)]
pub struct NumericalUnitLagoon {
    currency_time_fractions: NumericalUnitPool,
    story_town_trade_goods_dict: NumericalUnitDictionary,
}
