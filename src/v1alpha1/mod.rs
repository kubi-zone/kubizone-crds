mod record;
mod zone;

use std::{fmt::Display, str::FromStr};

pub use record::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
pub use zone::*;

#[derive(
    Serialize, Deserialize, Clone, Debug, JsonSchema, Hash, PartialEq, Eq, PartialOrd, Ord,
)]
pub struct ZoneRef {
    pub name: String,
    pub namespace: Option<String>,
}

impl ZoneRef {
    pub fn as_label(&self) -> String {
        if let Some(namespace) = &self.namespace {
            format!("{}.{namespace}", self.name)
        } else {
            self.name.clone()
        }
    }
}

impl From<&str> for ZoneRef {
    fn from(s: &str) -> Result<Self, Self::Err> {
        if let Ok((namespace, name)) = s.split_once('.') {
            Ok(ZoneRef {
                name: name.to_string(),
                namespace: Some(namespace.to_string()),
            })
        } else {
            Ok(ZoneRef {
                name: s.to_string(),
                namespace: None,
            })
        }
    }
}

impl From<&String> for ZoneRef {
    fn from(value: &String) -> Self {
        ZoneRef::from(value.as_deref())
    }
}

impl From<String> for ZoneRef {
    fn from(value: String) -> Self {
        ZoneRef::from(value.as_deref())
    }
}

impl Display for ZoneRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(namespace) = &self.namespace {
            write!(f, "{namespace}/{}", self.name)
        } else {
            f.write_str(&self.name)
        }
    }
}

pub mod defaults {
    use kubizone_common::Class;

    pub const CLASS: Class = Class::IN;
    pub(super) fn class() -> Class {
        CLASS
    }
}
