use image::{imageops, DynamicImage, GenericImage, GenericImageView, ImageBuffer, Rgba, RgbaImage};
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

	pub fn set_sprite(&mut self, sprite: DynamicImage, x: u32, y: u32) -> Result<()> {
		let (width, height) = sprite.dimensions();
		if x >= self.columns || y >= self.rows {
			return Err("Sprite X or Y outside of sheet bounds!".into());
		}
		if width > self.sprite_width || height > self.sprite_height {
			return Err("Sprite height or width outside of sheet bounds!".into());
		}
		let start_x = x * self.sprite_width;
		let start_y = y * self.sprite_height;
		for (sprite_x, sprite_y, pixel) in sprite.pixels() {
			let target_x = start_x + sprite_x;
			let target_y = start_y + sprite_y;
			self.image.put_pixel(target_x, target_y, pixel);
		}

		Ok(())
	}

	pub fn get_sprite(&self, x: u32, y: u32) -> Result<DynamicImage> {
		if x >= self.columns || y >= self.rows {
			return Err("Sprite X or Y outside of sheet bounds!".into());
		}
		let img = imageops::crop_imm(
			&self.image,
			x * self.sprite_width,
			y * self.sprite_height,
			self.sprite_width,
			self.sprite_height,
		);
		Ok(DynamicImage::ImageRgba8(img.to_image()))
	}
}

pub trait ToHashMap<T> {
	fn to_hashmap(
		&mut self, map: Vec<Vec<T>>, blank: T, trim_top: bool, trim_right: bool, trim_bottom: bool,
		trim_left: bool,
	) -> Result<Sprites<T>>;
}

enum Coord {
	C1,
	C2,
}

impl<T> ToHashMap<T> for SpriteSheet
where
	T: std::fmt::Debug + std::cmp::Eq + std::hash::Hash,
{
	fn to_hashmap(
		&mut self, map: Vec<Vec<T>>, blank: T, trim_top: bool, trim_right: bool, trim_bottom: bool,
		trim_left: bool,
	) -> Result<Sprites<T>> {
		let mut sprites = HashMap::new();
		for (y, row) in map.into_iter().enumerate() {
			for (x, ident) in row.into_iter().enumerate() {
				if ident == blank {
					continue;
				}
				let x = x as u32 * self.sprite_width;
				let y = y as u32 * self.sprite_height;
				let mut sprite = self
					.image
					.crop_imm(x, y, self.sprite_width, self.sprite_height);
				let width = self.sprite_width as i32 - 1;
				let height = self.sprite_height as i32 - 1;
				let x = match trim_left {
					true => get_trim(&sprite, 0, 0, 1, 1, Coord::C1)? as u32,
					false => 0,
				};
				let y = match trim_top {
					true => get_trim(&sprite, width, 0, -1, 1, Coord::C2)? as u32,
					false => 0,
				};
				let new_width = match trim_right {
					true => get_trim(&sprite, width, 0, -1, 1, Coord::C1)? as u32 + 1,
					false => self.sprite_width,
				};
				let new_height = match trim_bottom {
					true => get_trim(&sprite, width, height, -1, -1, Coord::C2)? as u32 + 1,
					false => self.sprite_height,
				};
				sprite = sprite.crop_imm(x, y, new_width, new_height);
				sprites.insert(ident, sprite);
			}
		}
		Ok(sprites)
	}
}

fn get_trim(
	sprite: &DynamicImage, mut c1: i32, mut c2: i32, c1_shift: i32, c2_shift: i32, coord: Coord,
) -> Result<i32> {
	let c2_start = c2;
	loop {
		loop {
			if sprite.get_pixel(c1 as u32, c2 as u32) != Rgba([0, 0, 0, 0]) {
				return match coord {
					Coord::C1 => Ok(c1),
					Coord::C2 => Ok(c2),
				};
			}
			c2 += c2_shift;
			if c2 < 0 || c2 as u32 >= sprite.dimensions().1 {
				c2 = c2_start;
				break;
			}
		}
		c1 += c1_shift;
		if c1 < 0 || c1 as u32 >= sprite.dimensions().0 {
			break;
		}
	}
	Err("Sprite is empty!".into())
}
