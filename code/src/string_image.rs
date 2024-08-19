use crate::color::Color;
use crate::error::Result;
use crate::fs::find_files_in_dir;
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba, RgbaImage};
use std::collections::HashMap;
use std::path::MAIN_SEPARATOR as slash;

#[derive(Debug)]
pub struct StringImage {
	pub chars: HashMap<char, DynamicImage>,
	pub line_height: u32,
	pub justification: Justification,
	pub border: Border,
	pub spacing: Spacing,
	pub colors: Colors,
	pub drop_shadow: Option<DropShadow>,
}

#[derive(Debug)]
pub enum Justification {
	Left,
	CenterBreakLeft,
	CenterBreakRight,
	Right,
}

#[derive(Debug)]
pub struct Border {
	pub top: u32,
	pub right: u32,
	pub bottom: u32,
	pub left: u32,
}

#[derive(Debug)]
pub struct Spacing {
	pub letter: u32,
	pub line: u32,
}

#[derive(Debug)]
pub struct Colors {
	pub text: Color,
	pub background: Color,
}

#[derive(Debug)]
pub struct DropShadow {
	pub color: Color,
	pub offset_x: i32,
	pub offset_y: i32,
}

impl StringImage {
	pub fn new(letter_dir: &str, ext: &str) -> Result<StringImage> {
		let font_files = find_files_in_dir(letter_dir, true)?;
		let font_files = font_files
			.iter()
			.filter(|f| f.ends_with(ext))
			.collect::<Vec<_>>();
		if font_files.is_empty() {
			panic!("No files found!");
		}
		let mut line_height: Option<u32> = None;
		let mut chars = HashMap::new();
		for char in font_files {
			let hex = char.trim_end_matches(ext).split(slash).last().unwrap();
			let image = image::open(char)?;
			if line_height.is_none() {
				line_height = Some(image.dimensions().1)
			} else if line_height != Some(image.dimensions().1) {
				panic!("Char height variance detected!");
			}
			chars.insert(
				char::from_u32(u32::from_str_radix(hex, 16)?).unwrap(),
				image,
			);
		}
		Ok(StringImage {
			chars,
			line_height: line_height.unwrap(),
			justification: Justification::CenterBreakLeft,
			border: Border::default(),
			spacing: Spacing::default(),
			colors: Colors::default(),
			drop_shadow: None,
		})
	}

	pub fn set_justification(mut self, justification: Justification) -> Self {
		self.justification = justification;
		self
	}

	pub fn set_border(mut self, top: u32, right: u32, bottom: u32, left: u32) -> Self {
		self.border = Border {
			top,
			right,
			bottom,
			left,
		};
		self
	}

	pub fn set_spacing(mut self, letter: u32, line: u32) -> Self {
		self.spacing = Spacing { letter, line };
		self
	}

	pub fn set_colors(mut self, text: Color, background: Color) -> Self {
		self.colors = Colors { text, background };
		self
	}

	pub fn set_drop_shadow(mut self, color: Color, offset_x: i32, offset_y: i32) -> Self {
		self.drop_shadow = Some(DropShadow {
			color,
			offset_x,
			offset_y,
		});
		self
	}

	pub fn text_to_image(&self, text: &str, filepath: Option<&str>) -> Result<DynamicImage> {
		let lines = text.split('\n').collect::<Vec<_>>();
		let height = (self.line_height * lines.len() as u32)
			+ (self.border.top + self.border.bottom)
			+ (lines.len() as u32 - 1) * self.spacing.line;
		let line_widths = lines
			.iter()
			.map(|line| {
				line.chars()
					.map(|c| self.chars.get(&c).unwrap().dimensions().0)
					.sum::<u32>() + ((line.chars().count() as u32 - 1) * self.spacing.letter)
			})
			.collect::<Vec<_>>();
		let max_width = line_widths.iter().max().unwrap() + (self.border.left + self.border.right);

		let mut image: RgbaImage = ImageBuffer::from_fn(max_width, height, |_, _| {
			image::Rgba([
				self.colors.background.rgba.red,
				self.colors.background.rgba.green,
				self.colors.background.rgba.blue,
				self.colors.background.rgba.alpha,
			])
		});

		lines
			.iter()
			.zip(line_widths)
			.enumerate()
			.for_each(|(index, (line, width))| {
				let index = index as u32;
				let start_y =
					self.border.top + (index * self.line_height) + (index * self.spacing.line);
				let mut start_x = match self.justification {
					Justification::Left => self.border.left,
					Justification::CenterBreakLeft => {
						((max_width - self.border.left - self.border.right - width) as f64 / 2.0)
							.floor() as u32 + self.border.left
					}
					Justification::CenterBreakRight => {
						((max_width - self.border.left - self.border.right - width) as f64 / 2.0)
							.ceil() as u32 + self.border.left
					}
					Justification::Right => max_width - width - self.border.right,
				};

				for char in line.chars() {
					let char_image = self.chars.get(&char).unwrap();
					for pixel in char_image.pixels() {
						if pixel.2 .0[3] == 0 {
							continue;
						}
						let (x, y) = (pixel.0 + start_x, pixel.1 + start_y);
						let rgba = [
							self.colors.text.rgba.red,
							self.colors.text.rgba.green,
							self.colors.text.rgba.blue,
							self.colors.text.rgba.alpha,
						];
						image.put_pixel(x, y, Rgba(rgba));
					}
					start_x += char_image.dimensions().0 + self.spacing.letter;
				}
			});

		if let Some(filepath) = filepath {
			image.save(filepath)?;
		}

		Ok(DynamicImage::from(image))
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
		Spacing { letter: 1, line: 1 }
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

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn load_chars() -> Result<()> {
		StringImage::new("../archive/image-fonts/3x5-digits-square/", ".png")?;
		Ok(())
	}
}
