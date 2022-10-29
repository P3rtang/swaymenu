mod window;
mod custom_widgets;

use gtk::prelude::*;
use gtk::{gio, glib, Application, CssProvider, StyleContext};
use gtk::gdk::{Display};
use window::Window;
use debug_print::debug_println;
use gio::SimpleAction;
use glib::clone;

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
    app.set_accels_for_action("win.minimize", &["Escape"]);

    // Run the application
    app.run();
}

fn build_ui(app: &Application) {
    debug_println!("[INFO]: creating window");
    println!("{}", app.windows().len());
    if app.windows().len() == 0 {
        // Create new window and present it
        let window = Window::new(app);
        debug_println!("[INFO]: showing window");
        window.present();

        let action_minimize = SimpleAction::new("minimize", None);
        action_minimize.connect_activate(clone!(@weak window => move |_, _| {
            window.hide();
        }));
        window.add_action(&action_minimize);

        debug_println!("[INFO] trying to get the monitor resolution");
        let display = Display::default().expect("Could not connect to a display");
        let geometry = display.monitor_at_surface(&window.surface()).geometry();
        window.set_height_request(geometry.height());
        window.set_width_request(geometry.width());
        debug_println!("[DEBUG] setting resolution to {}x{}", geometry.width(), geometry.height());

        window.add_css_class("window");
        window.hide();
    } else {
        app.windows().get(0).unwrap().show();
    }
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