use super::threads::multithread;
use sha2::{Digest, Sha256};
use std::path::MAIN_SEPARATOR as SLASH;
use std::{fs, io};

pub fn get_hashes(files: Vec<String>) -> Vec<(String, String)> {
	multithread(files, None, |thread_num, file| {
		println!(
			"[thread {thread_num:02}] getting hash of file: {}",
			file.split(SLASH).last().unwrap()
		);

		let hash = get_hash(&file, false);
		Some((hash, file))
	})
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
