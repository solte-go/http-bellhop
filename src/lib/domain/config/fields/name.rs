use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Name(String);

impl Default for Name {
    fn default() -> Self {
        Self("bellhop-request".to_owned())
    }
}

impl Name {
    pub fn into_inner(self) -> String {
        self.0
    }
}
