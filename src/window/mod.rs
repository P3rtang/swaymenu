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
        let mut original_state = 0;

        let current_lock_state = Command::new("ps").arg("-ef").output().expect("failed to execute command");
        let mut sway_idle_count = 0;
        for out_item in String::from_utf8(current_lock_state.stdout) {
            if out_item.contains("swayidle") { sway_idle_count += 1 }
        }
        if sway_idle_count != 0 {
            label.set_label("");
        } else {
            label.set_label("");
            original_state = 1;
        }

        let action_lock_toggle = SimpleAction::new_stateful(
            "lockToggle",
            Some(&i32::static_variant_type()),
            &original_state.to_variant(),
        );

        action_lock_toggle.connect_activate(clone!(@weak button => move |action, parameter| {
            // Get state
            let mut state = action.state().expect("Could not get state.").get::<i32>().expect("The value needs to be of type `i32`.");
            // Get parameter
            let parameter = parameter.expect("Could not get parameter.").get::<i32>().expect("The value needs to be of type `i32`.");

            // Increase state by parameter and save state
            state += parameter;
            state %= 2;
            action.set_state(&state.to_variant());

            // Update label with new state
            if state == 0 {
                label.set_label("");
                Command::new("sh").arg("/home/p3rtang/.config/waybar/swayidle.sh").spawn().expect("failed to execute process")
            } else {
                label.set_label("");
                Command::new("killall").arg("swayidle").spawn().expect("failed to execute process")
            };
        }));
        self.add_action(&action_lock_toggle);
    }
}