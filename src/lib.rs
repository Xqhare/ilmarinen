
mod json;
mod lexical_unit_pool;

use ::json::JsonValue;

use crate::json::read_json;

use crate::lexical_unit_pool::LexicalUnitPool;


#[derive(Debug, Clone)]
pub struct WordSmith {
    library_adj: LexicalUnitPool,
}

impl Default for WordSmith {
    fn default() -> Self {
        let gen_lib_path = "lib/gen_lib.json";
        WordSmith { library_adj: LexicalUnitPool::default() }
    }
}

impl From<JsonValue> for WordSmith {
    fn from(value: JsonValue) -> Self {
        todo!()
    }
}

impl WordSmith {
    pub fn test_main() {
        let a = read_json("lib/gen_lib.json");
        println!("{:?}", a);
    }
}
