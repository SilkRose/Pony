use pdt::pdtfs::find_files_in_dir;

fn main() {
	let extensions = Some(vec![".md".to_string()]);
	let files = find_files_in_dir("../", true, &extensions);
	for file in files.iter() {
		println!("{file}")
	}
}
