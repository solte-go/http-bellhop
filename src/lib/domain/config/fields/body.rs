use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body(String);

impl Default for Body {
    fn default() -> Self {
        Self("test string".to_string())
    }
}

impl Body {
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl From<Body> for String {
    fn from(value: Body) -> Self {
        value.0
    }
}
