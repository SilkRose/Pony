use golden_oak_library::rarity::count_occurances;

fn main() {
	let text = "Pinkie Pie is best pony!".to_string();
	let count = count_occurances(text, "Pinkie".to_string());
	println!("{count}");
}
