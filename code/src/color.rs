use serde::{Deserialize, Serialize};

type Result<T, E = Box<dyn (::std::error::Error)>> = ::std::result::Result<T, E>;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Color {
	pub hex: String,
	pub rgba: Rgba,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Rgba {
	pub red: u8,
	pub green: u8,
	pub blue: u8,
	pub alpha: u8,
}

impl Color {
	pub fn from_hex(hex: String) -> Result<Color> {
		if !hex.starts_with("#") || hex.len() != 7 {
			return Err("Hex failed to parse!".into());
		}
		let rgba = Rgba {
			red: u8::from_str_radix(&hex[1..3], 16)?,
			green: u8::from_str_radix(&hex[3..5], 16)?,
			blue: u8::from_str_radix(&hex[5..7], 16)?,
			alpha: 255,
		};
		Ok(Color { hex, rgba })
	}
	pub fn from_rgba(red: u8, green: u8, blue: u8, alpha: u8) -> Result<Color> {
		let rgba = Rgba {
			red,
			green,
			blue,
			alpha,
		};
		let hex = format!("#{red:02X}{green:02X}{blue:02X}");
		Ok(Color { hex, rgba })
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn hex() {
		let hex = "#F5B7D0".to_string();
		let rgba = Rgba {
			red: 245,
			green: 183,
			blue: 208,
			alpha: 255,
		};
		let color = Color::from_hex(hex.clone()).unwrap();
		assert_eq!(Color { hex, rgba }, color)
	}
	#[test]
	#[should_panic]
	fn hex_missing_hash() {
		let hex = "F5B7D0".to_string();
		Color::from_hex(hex).unwrap();
	}
	#[test]
	#[should_panic]
	fn hex_short() {
		let hex = "#F5B".to_string();
		Color::from_hex(hex).unwrap();
	}
	#[test]
	fn rgba() {
		let hex = "#F5B7D0".to_string();
		let rgba = Rgba {
			red: 245,
			green: 183,
			blue: 208,
			alpha: 255,
		};
		let color = Color::from_rgba(rgba.red, rgba.green, rgba.blue, rgba.alpha).unwrap();
		assert_eq!(Color { hex, rgba }, color)
	}
}
