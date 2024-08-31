use image::{DynamicImage, GenericImage, GenericImageView, ImageBuffer, Rgba, RgbaImage};
use std::collections::HashMap;

type Result<T, E = Box<dyn (::std::error::Error)>> = ::std::result::Result<T, E>;
type Sprites<T> = HashMap<T, DynamicImage>;

#[derive(Debug)]
pub struct SpriteSheet {
	image: DynamicImage,
	sprite_width: u32,
	sprite_height: u32,
	rows: u32,
	columns: u32,
}

impl SpriteSheet {
	pub fn new(width: u32, height: u32, rows: u32, columns: u32) -> Result<SpriteSheet> {
		if rows == 0 || columns == 0 {
			return Err("Number of rows or columns cannot be zero!".into());
		}
		let image: RgbaImage = ImageBuffer::from_fn(width, height, |_, _| Rgba([0, 0, 0, 0]));
		let sprite_width = width / columns;
		let sprite_height = height / rows;
		if sprite_width == 0 || sprite_height == 0 {
			return Err("Sprite width or height is zero!".into());
		}
		if width % columns != 0 || height % rows != 0 {
			return Err("Image dimensions not divisible by number of rows or columns!".into());
		}
		Ok(SpriteSheet {
			image: DynamicImage::from(image),
			sprite_width,
			sprite_height,
			rows,
			columns,
		})
	}

	pub fn load(file: &str, rows: u32, columns: u32) -> Result<SpriteSheet> {
		if rows == 0 || columns == 0 {
			return Err("Number of rows or columns cannot be zero!".into());
		}
		let image = image::open(file)?;
		let (width, height) = image.dimensions();
		let sprite_width = width / columns;
		let sprite_height = height / rows;
		if sprite_width == 0 || sprite_height == 0 {
			return Err("Sprite width or height is zero!".into());
		}
		if width % columns != 0 || height % rows != 0 {
			return Err("Image dimensions not divisible by number of rows or columns!".into());
		}
		Ok(SpriteSheet {
			image,
			sprite_width,
			sprite_height,
			rows,
			columns,
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn load_image() -> Result<()> {
		let sprite_sheet = SpriteSheet::load("", 16, 16)?;
		Ok(())
	}
}
