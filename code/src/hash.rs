use sha2::{Digest, Sha256, Sha512};
use std::{fs, io};

type Result<T, E = Box<dyn (::std::error::Error)>> = ::std::result::Result<T, E>;

pub fn get_hash_sha256(filename: &str) -> Result<String> {
	let mut file = fs::File::open(filename)?;
	let mut hasher = Sha256::new();
	io::copy(&mut file, &mut hasher)?;
	Ok(format!("{:x}", hasher.finalize()))
}

pub fn get_hash_sha512(filename: &str) -> Result<String> {
	let mut file = fs::File::open(filename)?;
	let mut hasher = Sha512::new();
	io::copy(&mut file, &mut hasher)?;
	Ok(format!("{:x}", hasher.finalize()))
}
