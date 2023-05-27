use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Clone, Serialize, PartialEq)]
pub enum Name {
    Default,
    Dev,
    Stage,
    Lab,
    Prod,
}

impl<'de> Deserialize<'de> for Name {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(match s.to_lowercase().as_str() {
            "dev" => Name::Dev,
            "stage" => Name::Stage,
            "lab" => Name::Lab,
            "prod" => Name::Prod,
            _ => Name::Default,
        })
    }
}

impl Default for Name {
    fn default() -> Self {
        Name::Default
    }
}

impl Name {
    pub fn into_inner(self) -> String {
        self.into()
    }

    pub fn new() -> Option<Self> {
        Option::Some(Name::Default)
    }
}

impl From<Name> for String {
    fn from(value: Name) -> Self {
        match value {
            Name::Default => "default".to_owned(),
            Name::Dev => "deb".to_owned(),
            Name::Stage => "stage".to_owned(),
            Name::Lab => "lab".to_owned(),
            Name::Prod => "prod".to_owned(),
        }
    }
}
