use std::fmt;

use serde::de::{self, Deserializer, Visitor};

pub mod board;
pub mod catalog;
pub mod client;
pub mod error;
pub mod post;
pub mod prelude;
pub mod thread;

pub type Result<T> = ::std::result::Result<T, crate::error::Error>;

pub(crate) const BASE: &str = "https://a.4cdn.org/";

pub(crate) fn int_to_bool<'de, D>(data: D) -> ::std::result::Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    struct Visit;
    impl<'de> Visitor<'de> for Visit {
        type Value = bool;
        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            write!(formatter, "an integer")
        }

        fn visit_i64<E: de::Error>(self, n: i64) -> ::std::result::Result<Self::Value, E> {
            Ok(n != 0)
        }
    }

    data.deserialize_i64(Visit {})
}
