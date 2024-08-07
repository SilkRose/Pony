use super::error::Result;
use crate::fs::find_files_in_dir;
use image::DynamicImage;
use std::collections::HashMap;
use std::path::Path;

pub struct CharSet {
	pub chars: HashMap<char, DynamicImage>,
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
		Ok(CharSet { chars })
	}
}
