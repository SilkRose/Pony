use gtk::{glib, prelude::*, Application, ApplicationWindow, Box, Button, Entry, Orientation};
const APP_ID: &str = "dev.silkrose.test-ui";

fn main() -> glib::ExitCode {
	let app = Application::builder().application_id(APP_ID).build();

	app.connect_activate(build_ui);

	app.run()
}

fn build_ui(app: &Application) {
	let title = Entry::builder()
		.max_length(80)
		.margin_top(24)
		.margin_bottom(12)
		.margin_start(12)
		.margin_end(12)
		.build();

	let button = Button::builder()
		.label("Button")
		.margin_top(12)
		.margin_bottom(12)
		.margin_start(12)
		.margin_end(12)
		.build();

	button.connect_clicked(|button| {
		button.set_label("Test");
	});

	let content = Box::new(Orientation::Vertical, 0);
	content.append(&title);
	content.append(&button);

	let window = ApplicationWindow::builder()
		.application(app)
		.title("My GTK App")
		.width_request(320)
		.height_request(240)
		.child(&content)
		.build();

	window.present();
}
