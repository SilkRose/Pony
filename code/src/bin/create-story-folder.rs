use gtk::{
	glib, prelude::*, Application, ApplicationWindow, Box, Button, Entry, Label, Orientation,
	TextBuffer, TextView,
};
const APP_ID: &str = "dev.silkrose.create-story-folder";

fn main() -> glib::ExitCode {
	let app = Application::builder().application_id(APP_ID).build();

	app.connect_activate(build_ui);

	app.run()
}

fn build_ui(app: &Application) {
	let title_label = Label::builder().label("Title:").margin_top(12).build();

	let title = Entry::builder()
		.max_length(80)
		.margin_top(12)
		.margin_bottom(12)
		.margin_start(12)
		.margin_end(12)
		.build();

	let synopsis_label = Label::builder().label("Synopsis:").build();

	let synopsis_text = TextBuffer::builder().build();
	let synopsis = TextView::builder()
		.buffer(&synopsis_text)
		.pixels_above_lines(2)
		.pixels_below_lines(2)
		.pixels_inside_wrap(2)
		.height_request(66)
		.bottom_margin(4)
		.top_margin(4)
		.left_margin(4)
		.right_margin(4)
		.wrap_mode(gtk::WrapMode::Word)
		.margin_top(12)
		.margin_bottom(12)
		.margin_start(12)
		.margin_end(12)
		.build();

	let button = Button::builder()
		.label("Create Story Folder")
		.margin_top(12)
		.margin_bottom(12)
		.margin_start(12)
		.margin_end(12)
		.build();

	button.connect_clicked(|button| {
		button.set_label("Test");
	});

	let content = Box::new(Orientation::Vertical, 0);
	content.append(&title_label);
	content.append(&title);
	content.append(&synopsis_label);
	content.append(&synopsis);
	content.append(&button);

	let window = ApplicationWindow::builder()
		.application(app)
		.title("Create Story Folder")
		.width_request(400)
		.height_request(200)
		.child(&content)
		.build();

	window.present();
}
