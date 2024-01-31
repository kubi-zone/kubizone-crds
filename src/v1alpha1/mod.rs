mod record;
mod zone;

use std::fmt::Display;

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
