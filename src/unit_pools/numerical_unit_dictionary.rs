use json::JsonValue;


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

impl From<(&str, &JsonValue)> for NumericalUnitDictionary {
    fn from(value: (&str, &JsonValue)) -> Self {
        let name = value.0.to_string();
        let mut data: Vec<NumericalUnitDictionaryEntry> = Default::default();
        for entry in value.1.entries() {
            data.push(NumericalUnitDictionaryEntry::from(entry));
        }
        NumericalUnitDictionary { name, data }
    }
}
   

#[derive(Debug, Clone)]
pub struct NumericalUnitDictionaryEntry {
    pub name: String,
    pub data: f32,
}

impl Default for NumericalUnitDictionaryEntry {
    fn default() -> Self {
        NumericalUnitDictionaryEntry { name: "Default_Numerical_Unit_Dictionary_Entry".to_string(), data: 0.0 }
    }
}

impl From<(&str, &JsonValue)> for NumericalUnitDictionaryEntry {
    fn from(value: (&str, &JsonValue)) -> Self {
        NumericalUnitDictionaryEntry { name: value.0.to_string(), data: value.1.as_f32().expect("Wrong json entry! Has to be f32!") }
    }
}
