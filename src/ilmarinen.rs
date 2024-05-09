use json::JsonValue;

use crate::{lexical_unit_pool::LexicalUnitPool, json::read_json};


#[derive(Debug, Clone)]
pub struct Ilmarinen {
    library_adj: LexicalUnitPool,
}

impl Default for Ilmarinen {
    fn default() -> Self {
        Ilmarinen { library_adj: LexicalUnitPool::default() }
    }
}

impl From<JsonValue> for Ilmarinen {
    fn from(value: JsonValue) -> Self {
        todo!()
    }
}

impl Ilmarinen {
    pub fn test_main() {
        let a = read_json("lib/gen_lib.json");
        println!("{:?}", a);
    }
}
