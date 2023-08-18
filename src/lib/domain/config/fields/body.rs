use serde::{Deserialize, Serialize};

/// `Body` is a public structure that can be serialized, deserialized, debugged, and cloned.
/// This structure holds a `String` value. Being public, it is externally accessible and
/// it provides support for data conversion into JSON and from JSON, 
/// making it suitable for sending and receiving over a network or saving and loading into a file. 
/// It also supports debugging and can be duplicated.
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
