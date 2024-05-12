
#[derive(Debug, Clone)]
pub struct NumericalUnitDictionary {
    pub name: String,
    pub data: Vec<NumericalUnitDictionaryEntry>
}

impl Default for NumericalUnitDictionary {
    fn default() -> Self {
        NumericalUnitDictionary { name: "Default_Numerical_Unit_Dictionary".to_string(), data: vec![NumericalUnitDictionaryEntry::default(), NumericalUnitDictionaryEntry::default()] }
    }
}
   

#[derive(Debug, Clone)]
pub struct NumericalUnitDictionaryEntry {
    pub name: String,
    pub entry: f32,
}

impl Default for NumericalUnitDictionaryEntry {
    fn default() -> Self {
        NumericalUnitDictionaryEntry { name: "Default_Numerical_Unit_Dictionary_Entry".to_string(), entry: 1.0 }
    }
}

impl NumericalUnitDictionaryEntry {
    pub fn new(name: String, entry: f32) -> NumericalUnitDictionaryEntry {
        NumericalUnitDictionaryEntry { name, entry }
    }
}
