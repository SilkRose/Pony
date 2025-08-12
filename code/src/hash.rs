use sha2::{Digest, Sha256, Sha512};
use std::fs;
use std::io::Read;
use std::num::NonZeroUsize;

type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;

const DEFAULT_CHUNK_SIZE: NonZeroUsize = NonZeroUsize::new(65536).unwrap();

pub fn get_hash_sha256(filename: &str, chunk_size: Option<NonZeroUsize>) -> Result<String> {
	let chunk_size = chunk_size.unwrap_or(DEFAULT_CHUNK_SIZE).into();
	let mut file = fs::OpenOptions::new().read(true).open(filename)?;
	let mut hasher = Sha256::new();
	let mut buf = vec![0u8; chunk_size];
	loop {
		let n = file.read(&mut buf)?;
		if n == 0 {
			break;
		}
		hasher.update(&buf[..n]);
	}
	Ok(format!("{:x}", hasher.finalize()))
}

pub fn get_hash_sha512(filename: &str, chunk_size: Option<NonZeroUsize>) -> Result<String> {
	let chunk_size = chunk_size.unwrap_or(DEFAULT_CHUNK_SIZE).into();
	let mut file = fs::OpenOptions::new().read(true).open(filename)?;
	let mut hasher = Sha512::new();
	let mut buf = vec![0u8; chunk_size];
	loop {
		let n = file.read(&mut buf)?;
		if n == 0 {
			break;
		}
		hasher.update(&buf[..n]);
	}
	Ok(format!("{:x}", hasher.finalize()))
}

pub fn get_hash_blake3(filename: &str, chunk_size: Option<NonZeroUsize>) -> Result<String> {
	let chunk_size = chunk_size.unwrap_or(DEFAULT_CHUNK_SIZE).into();
	let mut file = fs::OpenOptions::new().read(true).open(filename)?;
	let mut hasher = blake3::Hasher::new();
	let mut buf = vec![0u8; chunk_size];
	loop {
		let n = file.read(&mut buf)?;
		if n == 0 {
			break;
		}
		hasher.update(&buf[..n]);
	}
	Ok(hasher.finalize().to_hex().to_string())
}
