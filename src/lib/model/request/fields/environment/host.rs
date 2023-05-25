use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Host(String);

//TODO data from env
impl Default for Host {
    fn default() -> Self {
        Self("default".to_string())
    }
}

impl From<String> for Host {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Host {
    pub fn into_inner(self) -> String {
        self.0
    }
}
