use std::fmt;

use serde::de::{
	self,
	Deserializer,
	Visitor,
};

pub mod board;
pub mod post;

pub(crate) fn int_to_bool<'de, D>(data: D) -> Result<bool, D::Error>
where
	D: Deserializer<'de>,
{
	struct Visit;
	impl<'de> Visitor<'de> for Visit {
		type Value = bool;
		fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
			write!(formatter, "an integer")
		}

		fn visit_i64<E: de::Error>(self, n: i64) -> Result<Self::Value, E> {
			Ok(n != 0)
		}
	}

	data.deserialize_i64(Visit {})
}
