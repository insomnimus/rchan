use std::fmt;

use serde::de::{self, Visitor};

use super::*;

struct CapcodeVisitor;

impl<'de> Visitor<'de> for CapcodeVisitor {
    type Value = Capcode;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "one of [mod, admin, admin_highlight, manager, developer,, founder]"
        )
    }

    fn visit_str<E: de::Error>(self, s: &str) -> Result<Self::Value, E> {
        Ok(match s {
            "mod" => Capcode::Mod,
            "admin" => Capcode::Admin,
            "admin_highlight" => Capcode::AdminHighlight,
            "manager" => Capcode::Manager,
            "developer" => Capcode::Developer,
            "founder" => Capcode::Founder,
            _ => return Err(E::custom(format!("{} is not a valid capcode", s))),
        })
    }
}

impl<'de> serde::Deserialize<'de> for Capcode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(CapcodeVisitor)
    }
}
