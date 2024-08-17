use crate::color::Color;
use crate::error::Result;
use crate::fs::find_files_in_dir;
use image::{DynamicImage, GenericImageView};
use std::collections::HashMap;
use std::path::MAIN_SEPARATOR as slash;

#[derive(Debug)]
pub struct CharSet {
	pub chars: HashMap<char, DynamicImage>,
	pub line_height: u8,
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
	pub line: u8,
}

#[derive(Debug)]
pub struct Colors {
	pub text: Color,
	pub background: Color,
}

#[derive(Debug)]
pub struct DropShadow {
	pub color: Color,
	pub offset_x: i8,
	pub offset_y: i8,
	pub overlap_border: bool,
}

impl CharSet {
	pub fn new(letter_dir: &str, ext: &str) -> Result<CharSet> {
		let font_files = find_files_in_dir(letter_dir, true)?;
		let font_files = font_files
			.iter()
			.filter(|f| f.ends_with(ext))
			.collect::<Vec<_>>();
		if font_files.is_empty() {
			panic!("No files found!");
		}
		let mut line_height: Option<u8> = None;
		let mut chars = HashMap::new();
		for char in font_files {
			let hex = char.trim_end_matches(ext).split(slash).last().unwrap();
			let image = image::open(char)?;
			if line_height.is_none() {
				line_height = Some(image.dimensions().1 as u8)
			} else if line_height != Some(image.dimensions().1 as u8) {
				panic!("Char height variance detected!");
			}
			chars.insert(
				char::from_u32(u32::from_str_radix(hex, 16)?).unwrap(),
				image,
			);
		}
		Ok(CharSet {
			chars,
			line_height: line_height.unwrap(),
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

	pub fn set_spacing(&mut self, letter: u8, line: u8) -> &mut CharSet {
		self.spacing = Spacing { letter, line };
		self
	}

	pub fn set_colors(&mut self, text: Color, background: Color) -> &mut CharSet {
		self.colors = Colors { text, background };
		self
	}

	pub fn set_drop_shadow(
		&mut self, color: Color, offset_x: i8, offset_y: i8, overlap_border: bool,
	) -> &mut CharSet {
		self.drop_shadow = Some(DropShadow {
			color,
			offset_x,
			offset_y,
			overlap_border,
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
		Spacing { letter: 1, line: 2 }
	}
}

impl Default for Colors {
	fn default() -> Self {
		Colors {
			text: Color::from_rgba(0, 0, 0, 255).unwrap(),
			background: Color::from_rgba(0, 0, 0, 0).unwrap(),
		}
	}
}
