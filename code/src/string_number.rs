use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{fmt, ops};

pub struct StringNumber<T> {
	num: T,
}

struct StringNumberVisitor<T> {
	_marker: std::marker::PhantomData<T>,
}

macro_rules! string_number {
	($($T:ty: $visit:ident),+ $(,)?) => {
		$(
			impl fmt::Debug for StringNumber<$T> {
				fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
					fmt::Debug::fmt(&self.num, f)
				}
			}

			impl fmt::Display for StringNumber<$T> {
				fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
					fmt::Display::fmt(&self.num, f)
				}
			}

			impl ops::Deref for StringNumber<$T> {
				type Target = $T;

				fn deref(&self) -> &$T {
					&self.num
				}
			}

			impl ops::DerefMut for StringNumber<$T> {
				fn deref_mut(&mut self) -> &mut $T {
					&mut self.num
				}
			}

			impl StringNumber<$T> {
				pub fn to_number(&self) -> $T {
					self.num
				}
			}

			impl Serialize for StringNumber<$T> {
				fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
				where
					S: Serializer,
				{
					self.num.serialize(serializer)
				}
			}

			impl<'de> Deserialize<'de> for StringNumber<$T> {
				fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
				where
					D: Deserializer<'de>,
				{
					deserializer.deserialize_any(StringNumberVisitor::<$T> {
						_marker: std::marker::PhantomData,
					})
					.map(|num| Self { num })
				}
			}

			impl<'de> Visitor<'de> for StringNumberVisitor<$T> {
				type Value = $T;

				fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
					formatter.write_str("a number, or a number in a string")
				}

				visit_method!($visit v: $T { Ok(v) });
				visit_method!(visit_str v: &str { v.parse().map_err(Error::custom) });
				visit_method!(visit_bytes v: &[u8] {
					std::str::from_utf8(v)
						.map_err(Error::custom)
						.and_then(|s| s.parse().map_err(Error::custom))
					}
				);
			}
		)+
	};
}

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

string_number!(
	f32: visit_f32,
	f64: visit_f64,
	u8: visit_u8,
	u16: visit_u16,
	u32: visit_u32,
	u64: visit_u64,
	u128: visit_u128,
	i8: visit_i8,
	i16: visit_i16,
	i32: visit_i32,
	i64: visit_i64,
	i128: visit_i128,
);

#[cfg(test)]
mod tests {
	use super::*;
	use ops::DerefMut;

	#[derive(Deserialize, Debug)]
	struct NumberI32 {
		number: StringNumber<i32>,
	}

	#[derive(Deserialize, Debug)]
	struct NumberF64 {
		number: StringNumber<f64>,
	}

	// Tests for NumberF64
	#[test]
	fn number_f64() {
		let json = r#"{"number": 123.4}"#;
		let parsed: NumberF64 = serde_json::from_str(json).unwrap();
		assert_eq!(123.4, parsed.number.num);
	}

	#[test]
	fn negative_number_f64() {
		let json = r#"{"number": -123.4}"#;
		let parsed: NumberF64 = serde_json::from_str(json).unwrap();
		assert_eq!(-123.4, parsed.number.num);
	}

	#[test]
	fn number_string_f64() {
		let json = r#"{"number": "123.4"}"#;
		let parsed: NumberF64 = serde_json::from_str(json).unwrap();
		assert_eq!(123.4, parsed.number.num);
	}

	#[test]
	fn negative_number_string_f64() {
		let json = r#"{"number": "-123.4"}"#;
		let parsed: NumberF64 = serde_json::from_str(json).unwrap();
		assert_eq!(-123.4, parsed.number.num);
	}

	#[test]
	#[should_panic]
	fn string_fail() {
		let json = r#"{"number": "string"}"#;
		let _: NumberF64 = serde_json::from_str(json).unwrap();
	}

	#[test]
	#[should_panic]
	fn bool_fail() {
		let json = r#"{"number": true}"#;
		let _: NumberF64 = serde_json::from_str(json).unwrap();
	}

	#[test]
	fn number_debug() {
		let json = r#"{"number": 123.4}"#;
		let parsed: NumberF64 = serde_json::from_str(json).unwrap();
		let debug = format!("{:?}", parsed.number);
		assert_eq!("123.4", debug);
	}

	#[test]
	fn number_display() {
		let json = r#"{"number": 123.4}"#;
		let parsed: NumberF64 = serde_json::from_str(json).unwrap();
		let display = format!("{}", parsed.number);
		assert_eq!("123.4", display);
	}

	#[test]
	fn number_f64_deref() {
		let json = r#"{"number": 123.4}"#;
		let parsed: NumberF64 = serde_json::from_str(json).unwrap();
		assert_eq!(123.4, *parsed.number);
	}

	#[test]
	fn number_f64_deref_mut() {
		let json = r#"{"number": 123.4}"#;
		let mut parsed: NumberF64 = serde_json::from_str(json).unwrap();
		*parsed.number.deref_mut() = 1.0;
		assert_eq!(1.0, parsed.number.num);
	}

	#[test]
	fn number_f64_to_number() {
		let json = r#"{"number": 123.4}"#;
		let parsed: NumberF64 = serde_json::from_str(json).unwrap();
		assert_eq!(123.4, parsed.number.to_number());
	}

	#[test]
	fn number_f64_serialize() {
		let json = r#"{"number": 123.4}"#;
		let parsed: NumberF64 = serde_json::from_str(json).unwrap();
		let string = serde_json::to_string(&parsed.number).unwrap();
		assert_eq!("123.4", string);
	}

	// Tests for NumberI32
	#[test]
	fn number_i32() {
		let json = r#"{"number": 123}"#;
		let parsed: NumberI32 = serde_json::from_str(json).unwrap();
		assert_eq!(123, parsed.number.num);
	}

	#[test]
	fn negative_number_i32() {
		let json = r#"{"number": -123}"#;
		let parsed: NumberI32 = serde_json::from_str(json).unwrap();
		assert_eq!(-123, parsed.number.num);
	}

	#[test]
	fn number_string_i32() {
		let json = r#"{"number": "123"}"#;
		let parsed: NumberI32 = serde_json::from_str(json).unwrap();
		assert_eq!(123, parsed.number.num);
	}

	#[test]
	fn negative_number_string_i32() {
		let json = r#"{"number": "-123"}"#;
		let parsed: NumberI32 = serde_json::from_str(json).unwrap();
		assert_eq!(-123, parsed.number.num);
	}

	#[test]
	#[should_panic]
	fn string_fail_i32() {
		let json = r#"{"number": "string"}"#;
		let _: NumberI32 = serde_json::from_str(json).unwrap();
	}

	#[test]
	#[should_panic]
	fn bool_fail_i32() {
		let json = r#"{"number": true}"#;
		let _: NumberI32 = serde_json::from_str(json).unwrap();
	}

	#[test]
	fn number_debug_i32() {
		let json = r#"{"number": 123}"#;
		let parsed: NumberI32 = serde_json::from_str(json).unwrap();
		let debug = format!("{:?}", parsed.number);
		assert_eq!("123", debug);
	}

	#[test]
	fn number_display_i32() {
		let json = r#"{"number": 123}"#;
		let parsed: NumberI32 = serde_json::from_str(json).unwrap();
		let display = format!("{}", parsed.number);
		assert_eq!("123", display);
	}

	#[test]
	fn number_i32_deref() {
		let json = r#"{"number": 123}"#;
		let parsed: NumberI32 = serde_json::from_str(json).unwrap();
		assert_eq!(123, *parsed.number);
	}

	#[test]
	fn number_i32_deref_mut() {
		let json = r#"{"number": 123}"#;
		let mut parsed: NumberI32 = serde_json::from_str(json).unwrap();
		*parsed.number.deref_mut() = 1;
		assert_eq!(1, parsed.number.num);
	}

	#[test]
	fn number_i32_to_number() {
		let json = r#"{"number": 123}"#;
		let parsed: NumberI32 = serde_json::from_str(json).unwrap();
		assert_eq!(123, parsed.number.to_number());
	}

	#[test]
	fn number_i32_serialize() {
		let json = r#"{"number": 123}"#;
		let parsed: NumberI32 = serde_json::from_str(json).unwrap();
		let string = serde_json::to_string(&parsed.number).unwrap();
		assert_eq!("123", string);
	}
}
