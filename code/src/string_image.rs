use crate::color::Color;
use crate::fs::find_files_in_dir;
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba, RgbaImage};
use std::collections::HashMap;
use std::path::MAIN_SEPARATOR as slash;

type Result<T, E = Box<dyn (::std::error::Error)>> = ::std::result::Result<T, E>;

#[derive(Debug)]
pub struct StringImage {
	chars: HashMap<char, DynamicImage>,
	line_height: u32,
	justification: Justification,
	border: Border,
	spacing: Spacing,
	colors: Colors,
	drop_shadow: Option<DropShadow>,
}

#[derive(Debug)]
pub enum Justification {
	Left,
	CenterBreakLeft,
	CenterBreakRight,
	Right,
}

#[derive(Debug)]
struct Border {
	top: u32,
	right: u32,
	bottom: u32,
	left: u32,
}

#[derive(Debug)]
struct Spacing {
	tab: u32,
	line: u32,
	space: u32,
	letter: u32,
}

#[derive(Debug)]
struct Colors {
	text: Color,
	background: Color,
}

#[derive(Debug)]
struct DropShadow {
	color: Color,
	offset_x: i32,
	offset_y: i32,
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

	pub fn set_spacing(mut self, tab: u32, line: u32, space: u32, letter: u32) -> Self {
		self.spacing = Spacing {
			tab,
			line,
			space,
			letter,
		};
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
		let mut line_widths: Vec<u32> = vec![];
		for line in &lines {
			let mut width = 0;
			for char in line.chars() {
				if char == ' ' {
					width += self.spacing.space;
					continue;
				} else if char == '\t' {
					width += self.spacing.tab;
					continue;
				}
				width += self
					.chars
					.get(&char)
					.unwrap_or_else(|| panic!("Char in text not found in set!"))
					.dimensions()
					.0
			}
			let char_count = line.chars().filter(|c| *c != ' ' || *c != '\t').count() as u32;
			if char_count > 1 {
				width += (char_count - 1) * self.spacing.letter;
			}
			line_widths.push(width);
		}
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

				let mut previous: Option<char> = None;
				for char in line.chars() {
					if let Some(pre) = previous {
						if pre != ' ' && pre != '\t' && char == ' ' || char == '\t' {
							start_x -= self.spacing.letter;
							if char == ' ' {
								start_x += self.spacing.space;
								continue;
							} else if char == '\t' {
								start_x += self.spacing.tab;
								continue;
							}
						}
					}

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

						if let Some(drop_shadow) = &self.drop_shadow {
							let x = x as i32 + drop_shadow.offset_x;
							let y = y as i32 + drop_shadow.offset_y;
							if x >= 0
								&& x < max_width as i32 && y >= 0
								&& y < height as i32 && Rgba(rgba)
								!= *image.get_pixel(x as u32, y as u32)
							{
								let rgba = [
									drop_shadow.color.rgba.red,
									drop_shadow.color.rgba.green,
									drop_shadow.color.rgba.blue,
									drop_shadow.color.rgba.alpha,
								];
								image.put_pixel(x as u32, y as u32, Rgba(rgba));
							}
						}
					}
					start_x += char_image.dimensions().0 + self.spacing.letter;
					previous = Some(char);
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
		Spacing {
			tab: 12,
			line: 2,
			space: 3,
			letter: 1,
		}
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
