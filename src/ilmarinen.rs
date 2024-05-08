use json::JsonValue;


#[derive(Debug, Clone)]
pub struct Ilmarinen {
    library_adj: Vec<String>,
}

impl Default for Ilmarinen {
    fn default() -> Self {
        todo!()
    }
}

impl From<JsonValue> for Ilmarinen {
    fn from(value: JsonValue) -> Self {
        todo!()
    }
}
