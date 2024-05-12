use json::JsonValue;


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

impl From<(&str, &JsonValue)> for LexicalUnitPool {
    fn from(value: (&str, &JsonValue)) -> Self {
        let name = value.0.to_string();
        let mut unit_pool: Vec<String> = Default::default();
        for entry in value.1.members() {
            unit_pool.push(entry.to_string());
        }
        LexicalUnitPool {name, unit_pool}
    }
}
