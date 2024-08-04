extern crate proc_macro;

use camino::Utf8Path;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use regex::Regex;

#[proc_macro]
pub fn fanfic(_input: TokenStream) -> TokenStream {
	let dirs = find_dirs_in_dir("../stories/", false);
	let meta_regex = ::std::option::Option::Some(Regex::new("-meta.md$").unwrap());
	let mut tokens = TokenStream::new();
	for dir in dirs {
		let files = find_files_in_dir(&dir, false);
		let meta = files
			.iter()
			.filter(|f| matches(f, &meta_regex, &::std::option::Option::None))
			.collect::<Vec<_>>()
			.first()
			.unwrap()
			.to_string();
		let meta = ::std::fs::read_to_string(meta).unwrap();
		let mut short_description = String::new();
		let split_regex = Regex::new(r"(?m)^\s*##\s+").unwrap();
		let sections: Vec<&str> = split_regex.split(&meta).collect();
		let short_description_prefix = "Short Description:";
		let short_description = sections
			.iter()
			.find(|&&section| section.trim().starts_with(short_description_prefix))
			.map(|section| section.trim_start_matches(short_description_prefix).trim())
			.unwrap_or(&"");
		let ident = dir.replace("-", "_").replace("../stories/", "");
		let ident = format_ident!("{ident}");
		let token = quote! {
			#[doc = #short_description]
			pub mod #ident {}
		};
		tokens.extend::<proc_macro::TokenStream>(token.into());
	}
	tokens.into()
}

fn find_dirs_in_dir(dir: &str, recursive: bool) -> Vec<String> {
	let mut dirs = vec![];
	let paths = Utf8Path::read_dir_utf8(dir.into()).unwrap();
	for path in paths {
		let path = path.unwrap().path().to_string();
		let utf8_path = Utf8Path::new(&path);
		if utf8_path.is_dir() {
			dirs.push(path.clone());
			if recursive {
				dirs.extend(find_dirs_in_dir(&path, recursive))
			}
		}
	}
	dirs
}

fn find_files_in_dir(dir: &str, recursive: bool) -> Vec<String> {
	let mut files = vec![];
	let paths = Utf8Path::read_dir_utf8(dir.into()).unwrap();
	for path in paths {
		let path = path.unwrap().path().to_string();
		let utf8_path = Utf8Path::new(&path);
		if utf8_path.is_dir() && recursive {
			files.extend(find_files_in_dir(&path, recursive));
		} else if utf8_path.is_file() {
			files.push(path);
		}
	}
	files
}

fn matches(string: &str, includes: &Option<Regex>, excludes: &Option<Regex>) -> bool {
	if let Some(excludes) = excludes {
		if excludes.is_match(string) {
			return false;
		}
	}
	if let Some(includes) = includes {
		return includes.is_match(string);
	}
	true
}
