
#[derive(Debug, Clone, Default)]
pub struct MintingResult {
    pub result: Vec<String>
}

impl From<Vec<String>> for MintingResult {
    fn from(value: Vec<String>) -> Self {
        MintingResult { result: value }
    }
}

impl From<Vec<&str>> for MintingResult {
    fn from(value: Vec<&str>) -> Self {
        let mut data: Vec<String> = Default::default();
        for entry in value {
            data.push(entry.to_string());
        }
        MintingResult { result: data }
    }
}
