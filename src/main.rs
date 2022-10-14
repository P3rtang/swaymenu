use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};
use gtk::ffi::gtk_widget_get_style_context;

const APP_ID: &str = "org.gtk_rs.HelloWorld2";

fn main() {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn build_ui(app: &Application) {
    let button = Button::builder()
        .label("Exit Sway")
        .margin(10)
        .build();

    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&button)
        .build();


    // Present window
    window.present();
}