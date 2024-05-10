use self::{lexical_unit_lagoon::LexicalUnitLagoon, numerical_unit_lagoon::NumericalUnitLagoon};


mod lexical_unit_pool;
mod lexical_unit_lagoon;
mod numerical_unit_pool;
mod numerical_unit_dictionary;
mod numerical_unit_lagoon;

#[derive(Debug, Clone)]
pub struct UnitArchipelago {
    pub lexical_unit_lagoon: LexicalUnitLagoon,
    pub numerical_unit_lagoon: NumericalUnitLagoon,
}
