use mawu::XffValue;

#[derive(Debug, Clone)]
pub struct NumericalUnitDictionary {
    pub name: String,
    pub data: Vec<NumericalUnitDictionaryEntry>,
}

impl Default for NumericalUnitDictionary {
    fn default() -> Self {
        NumericalUnitDictionary {
            name: "Default_Numerical_Unit_Dictionary".to_string(),
            data: vec![
                NumericalUnitDictionaryEntry::default(),
                NumericalUnitDictionaryEntry::default(),
            ],
        }
    }
}

impl From<(&str, &XffValue)> for NumericalUnitDictionary {
    fn from(value: (&str, &XffValue)) -> Self {
        let name = value.0.to_string();
        let mut data: Vec<NumericalUnitDictionaryEntry> = Default::default();
        for (key, value) in value.1.as_object().expect("Must be an array").iter() {
            data.push(NumericalUnitDictionaryEntry::from((key.as_str(), value)));
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
        NumericalUnitDictionaryEntry {
            name: "Default_Numerical_Unit_Dictionary_Entry".to_string(),
            data: 0.0,
        }
    }
}

impl From<(&str, &XffValue)> for NumericalUnitDictionaryEntry {
    fn from(value: (&str, &XffValue)) -> Self {
        NumericalUnitDictionaryEntry {
            name: value.0.to_string(),
            data: value
                .1
                .as_number()
                .expect("Wrong json entry! Has to be f32!")
                .into_f64()
                .expect("Must be float") as f32,
        }
    }
}
