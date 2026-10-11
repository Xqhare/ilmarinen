use mawu::XffValue;

#[derive(Debug, Clone)]
pub struct NumericalUnitPool {
    pub name: String,
    pub unit_pool: Vec<f32>,
}

impl Default for NumericalUnitPool {
    fn default() -> Self {
        NumericalUnitPool {
            name: "Default_Numerical_unit_Pool".to_string(),
            unit_pool: vec![0.0, 1.0, 1.5, -2.0],
        }
    }
}

impl From<(&str, &XffValue)> for NumericalUnitPool {
    fn from(value: (&str, &XffValue)) -> Self {
        let name = value.0.to_string();
        let mut unit_pool: Vec<f32> = Default::default();
        for entry in value.1.as_array().expect("Must be an array") {
            let num = entry.as_number().expect("Wrong json entry! Has to be f32!");
            let val = num.into_f64().unwrap_or_else(|| {
                num.into_usize()
                    .map(|u| u as f64)
                    .or_else(|| num.into_isize().map(|i| i as f64))
                    .expect("Must be a number")
            }) as f32;
            unit_pool.push(val);
        }
        NumericalUnitPool { name, unit_pool }
    }
}
