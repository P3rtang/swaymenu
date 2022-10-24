mod window;
mod custom_widgets;

use std::thread::sleep;
use std::time::Duration;
use gtk::prelude::*;
use gtk::{gio, Application, CssProvider, StyleContext};
use gtk::gdk::{Display, Toplevel, ToplevelSize};
use window::Window;

const APP_ID: &str = "org.gtk_rs.SwayMenu";

fn main() {
    // Register and include resources
    gio::resources_register_include!("swaymenu.gresource")
        .expect("Failed to register resources.");

    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();
    // Connect to signals
    app.connect_startup(|_| load_css());
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
    window.set_height_request(1080);
    window.set_width_request(1920);
    window.add_css_class("window");
}

fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = CssProvider::new();
    provider.load_from_data(include_bytes!("../gtk-ui/style.css"));

    // Add the provider to the default screen
    StyleContext::add_provider_for_display(
        &Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}