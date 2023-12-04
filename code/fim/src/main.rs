use camino::Utf8Path;
use std::env;
use std::fs;

fn main() {
	let filename = &env::args().collect::<Vec<_>>()[1];
	if !filename.ends_with(".md") {
		panic!("File must be Markdown.")
	};
	let filepath = Utf8Path::new(filename);
	if Utf8Path::exists(filepath) {
		todo!()
	} else {
		let _ = fs::File::create(filepath);
	}
	println!("{:?}", env::current_dir());
	println!("{filepath:?}");
}
