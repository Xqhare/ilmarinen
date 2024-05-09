
pub mod ilmarinen;
mod json;
mod lexical_unit_pool;

use crate::ilmarinen::Ilmarinen;
use crate::json::read_json;

impl Ilmarinen {
    pub fn test_main() {
        let a = read_json("lib/gen_lib.json");
        println!("{:?}", a);
    }
}
