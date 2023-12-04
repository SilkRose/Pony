use camino::Utf8Path;
use std::env;

fn main() {
	let filename = &env::args().collect::<Vec<_>>()[1];
	let path = Utf8Path::exists(Utf8Path::new(filename));
	println!("{:?}", env::current_dir());
	println!("{path:?}");
}
