#[derive(Debug, Clone, Default)]
pub struct MintingResult {
    pub result: Vec<String>,
}

impl MintingResult {
    pub fn new() -> MintingResult {
        Default::default()
    }
    pub fn add_result<S: ToString>(&mut self, result: S) {
        self.result.push(result.to_string());
    }
    pub fn get_results(&self) -> &Vec<String> {
        &self.result
    }
    pub fn get_results_owned(&mut self) -> Vec<String> {
        self.result.clone()
    }
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
