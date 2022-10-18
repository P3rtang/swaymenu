mod window;
mod custom_widgets;

use gtk::prelude::*;
use gtk::{gio, Application};
use window::Window;

const APP_ID: &str = "org.gtk_rs.SwayMenu";

fn main() {
    // Register and include resources
    gio::resources_register_include!("swaymenu.gresource")
        .expect("Failed to register resources.");

    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Pressing escape closes the menu
    app.set_accels_for_action("window.close", &["Escape"]);

    // Run the application
    app.run();
}

fn build_ui(app: &Application) {
    // Create new window and present it
    let window = Window::new(app);
    window.present();
    window.set_height_request(1440);
    window.set_width_request(2560);
}