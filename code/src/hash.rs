use rayon::prelude::*;
use sha2::{Digest, Sha256};
use std::path::MAIN_SEPARATOR as SLASH;
use std::{fs, io};

pub fn get_hashes(files: Vec<String>, announce: bool) -> Vec<(String, String)> {
	files
		.par_iter()
		.map(|file| (get_hash(&file, announce), file))
		.collect()
}

pub fn get_hash(filename: &str, announce: bool) -> String {
	if announce {
		println!("Getting hash of file: {}", &filename);
	}
	let mut file = fs::File::open(filename).expect("Failed to open file.");
	let mut hasher = Sha256::new();
	io::copy(&mut file, &mut hasher).expect("Failed to copy file into writer.");
	let hash = hasher.finalize();
	format!("{hash:x}")
}
