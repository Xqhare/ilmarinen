
#[derive(Debug, Clone)]
pub struct NumericalUnitDictionary {
    name: String,
    data: Vec<NumericalUnitDictionaryEntry>
}

#[derive(Debug, Clone)]
pub struct NumericalUnitDictionaryEntry {
    name: String,
    entry: f32,
}

impl NumericalUnitDictionaryEntry {
    pub fn new(name: String, entry: f32) -> NumericalUnitDictionaryEntry {
        NumericalUnitDictionaryEntry { name, entry }
    }
}
