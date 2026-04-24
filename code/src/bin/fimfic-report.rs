use rustautogui::{MouseClick, RustAutoGui};
use std::error::Error;
use std::fs;
use std::thread::sleep;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
	let id_file = fs::read_to_string("../../../../no-chapter-stories.txt")?;
	let story_ids = id_file
		.lines()
		.map(|id| id.parse::<i32>().unwrap())
		.collect::<Vec<i32>>();
	let clicker = RustAutoGui::new(false)?;
	sleep(Duration::from_secs(4));
	for id in story_ids {
		let url = format!("https://www.fimfiction.net/report/story/{}", id);
		clicker.move_mouse_to_pos(860, 60, 0.3)?;
		clicker.click(MouseClick::LEFT)?;
		clicker.keyboard_input(&url)?;
		clicker.keyboard_command("enter")?;
		sleep(Duration::from_millis(500));
		clicker.move_mouse_to_pos(860, 410, 0.3)?;
		sleep(Duration::from_millis(400));
		clicker.click_down(MouseClick::LEFT)?;
		sleep(Duration::from_millis(400));
		clicker.move_mouse_to_pos(1055, 750, 0.4)?;
		sleep(Duration::from_millis(400));
		clicker.keyboard_command("tab")?;
		clicker.move_mouse_to_pos(1055, 650, 0.3)?;
		sleep(Duration::from_millis(400));
		clicker.double_click()?;
		clicker.keyboard_input("This story has no chapters.")?;
		sleep(Duration::from_millis(400));
		clicker.move_mouse_to_pos(685, 980, 0.3)?;
		sleep(Duration::from_millis(400));
		clicker.click(MouseClick::LEFT)?;
		sleep(Duration::from_millis(500));
	}
	Ok(())
}
