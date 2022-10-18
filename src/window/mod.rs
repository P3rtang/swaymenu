mod imp;

use glib::{Object, clone};
use gtk::subclass::prelude::*;
use gtk::prelude::*;
use gtk::{gio, glib, Application};
use gio::SimpleAction;
use std::process::Command;


glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &Application) -> Self {
        // Create new window
        Object::new(&[("application", app)]).expect("Failed to create Window")
    }
    fn setup_lock_button(&self) {
        let button = self.imp().button.get();
        let label = self.imp().button_label.get();
        let original_state = 0;
        let action_count = SimpleAction::new_stateful(
            "lockToggle",
            Some(&i32::static_variant_type()),
            &original_state.to_variant(),
        );

        action_count.connect_activate(clone!(@weak button => move |action, parameter| {
            // Get state
            let mut state = action
                .state()
                .expect("Could not get state.")
                .get::<i32>()
                .expect("The value needs to be of type `i32`.");

            // Get parameter
            let parameter = parameter
                .expect("Could not get parameter.")
                .get::<i32>()
                .expect("The value needs to be of type `i32`.");

            // Increase state by parameter and save state
            state += parameter;
            state %= 2;
            action.set_state(&state.to_variant());

            // Update label with new state
            let command = if state == 0 {
                label.set_label("");
                Command::new("sh")
                    .arg("/home/p3rtang/.config/waybar/swayidle.sh")
                    // .arg("-w")
                    // .arg("timeout").arg("30").arg("'swaylock -f -c 000000 && swaymsg \"output * dpms off\"'")
                    // .arg("resume").arg("'swaymsg \"output * dpms on\"'")
                    .spawn()
                    .expect("failed to execute process")
            } else {
                label.set_label("");
                Command::new("killall")
                    .arg("swayidle")
                    .spawn()
                    .expect("failed to execute process")
            };
            let command = command.stdout;
        }));
        self.add_action(&action_count);
    }
}