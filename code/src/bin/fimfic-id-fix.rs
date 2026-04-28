use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
	let id_file = fs::read_to_string("../../../../cc-non-voters.txt")?;
	let mut user_ids = id_file
		.lines()
		.map(|id| id.parse::<i32>().unwrap())
		.collect::<Vec<i32>>();
	user_ids.sort();
	let groups = user_ids.chunks(20).collect::<Vec<_>>();
	for group in groups {
		println!("Group len: {}", group.len());
		for id in group {
			let url = format!("https://www.fimfiction.net/user/{}/", id);
			println!("{url}");
		}
	}
	Ok(())
}
