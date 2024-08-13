use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{fmt, ops};

impl fmt::Debug for StringNumber {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::Debug::fmt(&self.num, f)
	}
}

impl fmt::Display for StringNumber {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::Display::fmt(&self.num, f)
	}
}

impl ops::Deref for StringNumber {
	type Target = f64;
	fn deref(&self) -> &f64 {
		&self.num
	}
}

impl ops::DerefMut for StringNumber {
	fn deref_mut(&mut self) -> &mut f64 {
		&mut self.num
	}
}

impl StringNumber {
	pub fn to_number(&self) -> f64 {
		self.num
	}
}

pub struct StringNumber {
	num: f64,
}

impl Serialize for StringNumber {
	fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		ser.serialize_f64(self.num)
	}
}

struct StringNumberVisitor;

impl<'de> Deserialize<'de> for StringNumber {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: Deserializer<'de>,
	{
		let num = deserializer.deserialize_any(StringNumberVisitor)?;
		Ok(Self { num })
	}
}

/// lazy
macro_rules! visit_method {
	($method:ident $param:ident: $type:ty $body:block) => {
		fn $method<E>(self, $param: $type) -> Result<Self::Value, E>
		where
			E: ::serde::de::Error,
		{
			$body
		}
	};
}

impl<'de> Visitor<'de> for StringNumberVisitor {
	type Value = f64;

	fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		formatter.write_str("a number, or a number in a string")
	}

	visit_method!(visit_i64 v: i64 { Ok(v as f64) });
	visit_method!(visit_i128 v: i128 { Ok(v as f64) });
	visit_method!(visit_u64 v: u64 { Ok(v as f64) });
	visit_method!(visit_u128 v: u128 { Ok(v as f64) });
	visit_method!(visit_f64 v: f64 { Ok(v) });
	visit_method!(visit_str v: &str { v.parse().map_err(Error::custom) });
	visit_method!(visit_bytes v: &[u8] {
		std::str::from_utf8(v)
			.map_err(Error::custom)
			.and_then(|s| s.parse().map_err(Error::custom))
	});

	fn visit_some<D>(self, de: D) -> Result<Self::Value, D::Error>
	where
		D: Deserializer<'de>,
	{
		de.deserialize_any(Self)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[derive(Deserialize)]
	struct Number {
		number: StringNumber,
	}

	#[test]
	fn number_f64() {
		let json = r#"{"number": 123.4}"#;
		let parsed: Number = serde_json::from_str(json).unwrap();
		assert_eq!(123.4, parsed.number.num);
	}
	#[test]
	fn negative_number_f64() {
		let json = r#"{"number": -123.4}"#;
		let parsed: Number = serde_json::from_str(json).unwrap();
		assert_eq!(-123.4, parsed.number.num);
	}
	#[test]
	fn number_u128() {
		let json = r#"{"number": 123}"#;
		let parsed: Number = serde_json::from_str(json).unwrap();
		assert_eq!(123.0, parsed.number.num);
	}
	#[test]
	fn negative_number_i128() {
		let json = r#"{"number": -123}"#;
		let parsed: Number = serde_json::from_str(json).unwrap();
		assert_eq!(-123.0, parsed.number.num);
	}
	#[test]
	fn number_string_f64() {
		let json = r#"{"number": "123.4"}"#;
		let parsed: Number = serde_json::from_str(json).unwrap();
		assert_eq!(123.4, parsed.number.num);
	}
	#[test]
	fn negative_number_string_f64() {
		let json = r#"{"number": "-123.4"}"#;
		let parsed: Number = serde_json::from_str(json).unwrap();
		assert_eq!(-123.4, parsed.number.num);
	}
	#[test]
	fn number_string_u128() {
		let json = r#"{"number": "123"}"#;
		let parsed: Number = serde_json::from_str(json).unwrap();
		assert_eq!(123.0, parsed.number.num);
	}
	#[test]
	fn negative_number_string_i128() {
		let json = r#"{"number": "-123"}"#;
		let parsed: Number = serde_json::from_str(json).unwrap();
		assert_eq!(-123.0, parsed.number.num);
	}
	#[test]
	#[should_panic]
	fn string_fail() {
		let json = r#"{"number": "string"}"#;
		let _: Number = serde_json::from_str(json).unwrap();
	}
}
