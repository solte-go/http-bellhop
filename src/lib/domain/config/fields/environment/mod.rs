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
    pub fn into_inner(self, env: &str) -> Result<String, EnvironmentError> {
        if env.to_lowercase() == "dev" {
            let ok: Option<&Config> = self.0.iter().find(|&v| v.name == Some(Name::Dev));
            return Ok(ok.unwrap().host.clone().into_inner());
        }
        if env.to_lowercase() == "stage" {
            let ok: Option<&Config> = self.0.iter().find(|&v| v.name == Some(Name::Stage));
            return Ok(ok.unwrap().host.clone().into_inner());
        }
        if env.to_lowercase() == "lab" {
            let ok: Option<&Config> = self.0.iter().find(|&v| v.name == Some(Name::Lab));
            return Ok(ok.unwrap().host.clone().into_inner());
        }
        if env.to_lowercase() == "prod" {
            let ok: Option<&Config> = self.0.iter().find(|&v| v.name == Some(Name::Prod));
            return Ok(ok.unwrap().host.clone().into_inner());
        }

        Err(EnvironmentError::NotFound)
    }

    // fn check_if_exist(self, env: Name) -> Result<String, EnvironmentError> {
    //     if let Some(value) = self.into_iter().next() {
    //         return if value.name.unwrap_or_default() == env {
    //             Ok(value.host.into_inner())
    //         } else {
    //             Err(EnvironmentError::NotFound)
    //         };
    //     }
    //     Err(EnvironmentError::NotFound)
    // }
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
