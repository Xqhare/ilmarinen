use std::{path::Path, io::{Error, ErrorKind}};

use crate::jisard::read_json;

use self::{lexical_unit_lagoon::LexicalUnitLagoon, numerical_unit_lagoon::NumericalUnitLagoon};


mod lexical_unit_pool;
mod lexical_unit_lagoon;
mod numerical_unit_pool;
mod numerical_unit_dictionary;
mod numerical_unit_lagoon;

const GEN_LIB_FILENAME: &str = "gen_lib.json";
const NUM_LIB_FILENAME: &str = "num_lib.json";

#[derive(Debug, Clone, Default)]
pub struct UnitArchipelago {
    pub lexical_unit_lagoon: LexicalUnitLagoon,
    pub numerical_unit_lagoon: NumericalUnitLagoon,
}

impl UnitArchipelago {
    pub fn new(data_path: &Path) -> Result<UnitArchipelago, Error> {
        let gen_lib_path = data_path.join(GEN_LIB_FILENAME);
        let gen_lib_path = data_path.join(NUM_LIB_FILENAME);

        let lexical_unit_lagoon = LexicalUnitLagoon::new(&gen_lib_path);

        
    }
}
