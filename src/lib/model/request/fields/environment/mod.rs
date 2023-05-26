pub mod name;

pub use name::Name;

pub mod host;

pub use host::Host;

use serde::{Deserialize, Serialize};

#[derive(thiserror::Error, Debug)]
#[error("wrong env argument")]
pub enum EnvironmentError {
    NotFound,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Configs(Vec<Config>);

impl Configs {
    pub fn into_inner(self, env: &str) -> String {
        if env.to_lowercase() == "dev" {
            let ok: Option<&Config> = self.0.iter().find(|&v| v.name == Some(Name::Dev));
            return ok.unwrap().host.clone().into_inner();
        }
        if env.to_lowercase() == "stage" {
            return match self.check_if_exist(Name::Stage) {
                Ok(v) => v,
                Err(e) => e.to_string(),
            };
        }
        if env.to_lowercase() == "lab" {
            let ok: Option<&Config> = self.0.iter().find(|&v| v.name == Some(Name::Lab));
            return ok.unwrap().host.clone().into_inner();
        }
        if env.to_lowercase() == "prod" {
            return match self.check_if_exist(Name::Prod) {
                Ok(v) => v,
                Err(e) => e.to_string(),
            };
        }

        "default".to_string()
    }

    fn check_if_exist(self, env: Name) -> Result<String, EnvironmentError> {
        if let Some(value) = self.into_iter().next() {
            return if value.name.unwrap_or_default() == env {
                Ok(value.host.into_inner())
            } else {
                Err(EnvironmentError::NotFound)
            };
        }
        Err(EnvironmentError::NotFound)
    }
}

impl IntoIterator for Configs {
    type Item = Config;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Default for Configs {
    fn default() -> Self {
        let default_config: Vec<Config> = vec![{
            Config {
                host: Host::default(),
                name: Name::new(),
            }
        }];
        Self(default_config)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub host: Host,
    pub name: Option<Name>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            host: Host::default(),
            name: Name::new(),
        }
    }
}

// impl IntoIterator<Item = i64> for TestStruct {
//     type Item = i64;
//     type IntoIter = std::vec::IntoIter<Self::Item>;
//
//     fn into_iter(self) -> Self::IntoIter {
//         self.fields.into_iter()
//     }
// }

trait TitleCase {
    fn title(&self) -> String;
}

impl TitleCase for &str {
    fn title(&self) -> String {
        if !self.is_ascii() || self.is_empty() {
            return String::from(*self);
        }
        let (head, tail) = self.split_at(1);
        head.to_uppercase() + tail
    }
}
