use mawu::XffValue;

#[derive(Debug, Clone)]
/// This is how a simple list of data is stored.
pub struct LexicalUnitPool {
    pub name: String,
    pub unit_pool: Vec<String>,
}

impl LexicalUnitPool {
    /// Extend the unit pool
    ///
    /// Does not change the name
    pub fn extend(&mut self, other: LexicalUnitPool) -> Self {
        self.unit_pool.extend(other.unit_pool);
        self.clone()
    }
    /// Create a new, empty LexicalUnitPool
    pub fn new(name: &str) -> Self {
        LexicalUnitPool {
            name: name.to_string(),
            unit_pool: Vec::new(),
        }
    }
}

impl Default for LexicalUnitPool {
    fn default() -> Self {
        let name = "Default_Lexical_Unit_Pool".to_string();
        let unit_pool = vec![
            "Default".to_string(),
            "Lexical".to_string(),
            "Unit".to_string(),
            "Pool".to_string(),
        ];
        LexicalUnitPool { name, unit_pool }
    }
}

impl From<(&str, &XffValue)> for LexicalUnitPool {
    fn from(value: (&str, &XffValue)) -> Self {
        let name = value.0.to_string();
        let mut unit_pool: Vec<String> = Default::default();
        for entry in value.1.as_array().expect("Must be an array") {
            unit_pool.push(
                entry
                    .as_string()
                    .expect("Wrong json entry! Has to be string!")
                    .clone(),
            );
        }
        LexicalUnitPool { name, unit_pool }
    }
}
