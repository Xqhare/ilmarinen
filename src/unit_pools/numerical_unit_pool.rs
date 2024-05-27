use json::JsonValue;


#[derive(Debug, Clone)]
pub struct NumericalUnitPool {
    pub name: String,
    pub unit_pool: Vec<f32>,
}

impl Default for NumericalUnitPool {
    fn default() -> Self {
        NumericalUnitPool { name: "Default_Numerical_unit_Pool".to_string(), unit_pool: vec![0.0, 1.0, 1.5, -2.0] }
    }
}

impl From<(&str, &JsonValue)> for NumericalUnitPool {
    fn from(value: (&str, &JsonValue)) -> Self {
        let name = value.0.to_string();
        let mut unit_pool: Vec<f32> = Default::default();
        for entry in value.1.members() {
            unit_pool.push(entry.as_f32().expect("Wrong json entry! Has to be f32!"));
        }
        NumericalUnitPool { name, unit_pool }
    }
}
