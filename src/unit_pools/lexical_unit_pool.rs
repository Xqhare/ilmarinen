
#[derive(Debug, Clone)]
/// This is how a simple list of data is stored.
pub struct LexicalUnitPool {
    pub name: String,
    pub unit_pool: Vec<String>,
}

impl Default for LexicalUnitPool {
    fn default() -> Self {
        let name = "Default_Lexical_Unit_Pool".to_string();
        let unit_pool = vec!["Default".to_string(), "Lexical".to_string(), "Unit".to_string(), "Pool".to_string()];
        LexicalUnitPool { name, unit_pool }
    }
}

impl From<(String, Vec<String>)> for LexicalUnitPool {
    fn from(value: (String, Vec<String>)) -> Self {
        LexicalUnitPool { name: value.0, unit_pool: value.1 }
    }
}
