
#[derive(Debug, Clone)]
pub struct NumericalUnitPool {
    pub name: String,
    pub data: Vec<f32>,
}

impl Default for NumericalUnitPool {
    fn default() -> Self {
        NumericalUnitPool { name: "Default_Numerical_unit_Pool".to_string(), data: vec![0.0, 1.0, 1.5, -2.0] }
    }
}
