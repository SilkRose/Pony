use super::error::Result;
use crate::fs::find_files_in_dir;
use image::DynamicImage;
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug)]
pub struct CharSet {
	pub chars: HashMap<char, DynamicImage>,
	pub justification: Justification,
	pub border: Border,
	pub spacing: Spacing,
	pub colors: Colors,
	pub drop_shadow: Option<DropShadow>,
}

#[derive(Debug)]
pub enum Justification {
	Left,
	Center,
	Right,
}

#[derive(Debug)]
pub struct Border {
	pub top: u8,
	pub right: u8,
	pub bottom: u8,
	pub left: u8,
}

#[derive(Debug)]
pub struct Spacing {
	pub letter: u8,
	pub space: u8,
	pub line: u8,
	pub tab: u8,
}

#[derive(Debug)]
pub struct Colors {
	pub text: Color,
	pub background: Color,
}

#[derive(Debug)]
pub struct Color {
	pub red: u8,
	pub green: u8,
	pub blue: u8,
	pub alpha: u8,
}

#[derive(Debug)]
pub struct DropShadow {
	pub color: Color,
	pub offset_x: i8,
	pub offset_y: i8,
}

impl CharSet {
	pub fn new(letter_dir: &str, ext: &str) -> Result<CharSet> {
		let font_files = find_files_in_dir(letter_dir, true)?;
		let font_files = font_files
			.iter()
			.filter(|f| f.ends_with(ext))
			.collect::<Vec<_>>();
		let mut chars = HashMap::new();
		for char in font_files {
			let ident = Path::new(char).file_stem().unwrap().to_str().unwrap();
			let image = image::open(char)?;
			chars.insert(char::from_u32(ident.parse::<u32>()?).unwrap(), image);
		}
		Ok(CharSet {
			chars,
			justification: Justification::Center,
			border: Border::default(),
			spacing: Spacing::default(),
			colors: Colors::default(),
			drop_shadow: None,
		})
	}

	pub fn set_justification(&mut self, justification: Justification) -> &mut CharSet {
		self.justification = justification;
		self
	}

	pub fn set_border(&mut self, top: u8, right: u8, bottom: u8, left: u8) -> &mut CharSet {
		self.border = Border {
			top,
			right,
			bottom,
			left,
		};
		self
	}

	pub fn set_spacing(&mut self, letter: u8, space: u8, line: u8, tab: u8) -> &mut CharSet {
		self.spacing = Spacing {
			letter,
			space,
			line,
			tab,
		};
		self
	}

	pub fn set_colors(&mut self, text: Color, background: Color) -> &mut CharSet {
		self.colors = Colors { text, background };
		self
	}

	pub fn set_drop_shadow(&mut self, color: Color, offset_x: i8, offset_y: i8) -> &mut CharSet {
		self.drop_shadow = Some(DropShadow {
			color,
			offset_x,
			offset_y,
		});
		self
	}
}

impl Default for Border {
	fn default() -> Self {
		Border {
			top: 1,
			right: 1,
			bottom: 1,
			left: 1,
		}
	}
}

impl Default for Spacing {
	fn default() -> Self {
		Spacing {
			letter: 1,
			space: 2,
			line: 2,
			tab: 4,
		}
	}
}

impl Default for Colors {
	fn default() -> Self {
		Colors {
			text: Color {
				red: 0,
				green: 0,
				blue: 0,
				alpha: 255,
			},
			background: Color {
				red: 0,
				green: 0,
				blue: 0,
				alpha: 0,
			},
		}
	}
}
