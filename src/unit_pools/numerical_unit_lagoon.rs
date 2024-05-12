use super::{numerical_unit_pool::NumericalUnitPool, numerical_unit_dictionary::NumericalUnitDictionary};


#[derive(Debug, Clone, Default)]
pub struct NumericalUnitLagoon {
    pub currency_time_fractions: NumericalUnitPool,
    pub story_town_trade_goods_dict: NumericalUnitDictionary,
}
