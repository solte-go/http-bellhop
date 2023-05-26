use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpectedStatus(String);

impl Default for ExpectedStatus {
    fn default() -> Self {
        Self("500".to_owned())
    }
}

impl ExpectedStatus {
    pub fn into_inner(self) -> String {
        self.0
    }
}
