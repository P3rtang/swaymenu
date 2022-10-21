mod imp;

use glib::{Object, clone};
use gtk::subclass::prelude::*;
use gtk::prelude::*;
use gtk::{gio, glib, Application, EventControllerMotion};
use std::process::Command;
use std::thread;
use std::time::Duration;


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
    // implement lock button
    fn setup_lock_button(&self) {
        let controller = EventControllerMotion::new();
        let info_label = self.imp().info_label.get();
        let lock_button = self.imp().lock_button.get();
        let lock_label = self.imp().lock_label.get();

        let current_lock_state = Command::new("ps").arg("-ef").output().expect("failed to execute command");
        let mut sway_idle_count = 0;
        for out_item in String::from_utf8(current_lock_state.stdout) {
            if out_item.contains("swayidle") { sway_idle_count += 1 }
        }
        println!("{}", sway_idle_count);
        if sway_idle_count != 0 {
            lock_label.set_label("");
            lock_button.set_css_classes(&["lock-closed"]);
            self.imp().lock_state.set(1);
        } else {
            lock_label.set_label("");
            lock_button.set_css_classes(&["lock-open"]);
        }

        lock_button.add_controller(&controller);
        controller.connect_enter(clone!(@weak self as window => move |_,_,_| {
            if window.imp().lock_state.get() == 1 {
                window.imp().info_label.set_label("Screen Timeout\nON");
            } else {
                info_label.set_label("Screen Timeout\nOFF");
            }
        }));
        controller.connect_leave(clone!(@weak self as window => move |_| {
            window.imp().info_label.set_label("");
        }));
    }
    // implement sway exit button
    fn setup_exit_button(&self) {
        let controller = EventControllerMotion::new();
        let info_label = self.imp().info_label.get();
        let exit_button = self.imp().exit_button.get();

        exit_button.connect_clicked(clone!(@weak exit_button, @weak info_label => move |_| {
            Command::new("swaymsg").arg("exit").output().expect("unable to exit sway");
        }));
        // adding a widget controller to show info on hover
        exit_button.add_controller(&controller);
        controller.connect_enter(clone!(@weak info_label => move |_,_,_| {
            info_label.set_label("Logout");
            info_label.set_css_classes(&["warning_hover"]);
        }));
        controller.connect_leave(clone!(@weak info_label => move |_| {
            info_label.remove_css_class("warning_hover");
            info_label.set_label("");
        }));
    }
    fn setup_shutdown_button(&self) {
        let controller = EventControllerMotion::new();
        let info_label = self.imp().info_label.get();
        let shutdown_button = self.imp().shutdown_button.get();

        // adding a widget controller to show info on hover
        shutdown_button.add_controller(&controller);
        controller.connect_enter(clone!(@weak info_label => move |_,_,_| {
            info_label.set_label("Shutdown");
            info_label.set_css_classes(&["warning_hover"]);
        }));
        controller.connect_leave(clone!(@weak info_label => move |_| {
            info_label.remove_css_class("warning_hover");
            info_label.set_label("");
        }));

        shutdown_button.connect_clicked(move |this| {
            this.add_css_class("warning_button_clicked");
            println!("Shutting down system");
            Command::new("systemctl").arg("poweroff").output().expect("unable to shutdown");
        });
    }
    fn setup_win_vm_button(&self) {
        let controller = EventControllerMotion::new();
        let info_label = self.imp().info_label.get();
        let win_button = self.imp().start_win_vm.get();

        // adding a widget controller to show info on hover
        win_button.add_controller(&controller);
        controller.connect_enter(clone!(@weak info_label => move |_,_,_| {
            info_label.set_label("Start Windows");
        }));
        controller.connect_leave(clone!(@weak info_label => move |_| {
            info_label.set_label("");
        }));

        // connecting the button signal on click
        win_button.connect_clicked(move |_| {
            println!("starting vm");
            thread::spawn(move || {
                Command::new("sh").arg("/home/p3rtang/IdeaProjects/swaymenu/scripts/start_vm.sh")
                    .output().expect("unable to start win11 vm");
                let five_seconds = Duration::from_secs(8);
                thread::sleep(five_seconds);
                Command::new("ddcutil").arg("setvcp").arg("60").arg("0x0f").output().unwrap();
            });
        });
    }
    fn setup_reboot_button(&self) {
        let controller = EventControllerMotion::new();
        let info_label = self.imp().info_label.get();
        let reboot_button = self.imp().reboot_button.get();

        // adding a widget controller to show info on hover
        reboot_button.add_controller(&controller);
        controller.connect_enter(clone!(@weak info_label => move |_,_,_| {
            info_label.set_label("Reboot");
            info_label.set_css_classes(&["warning_hover"]);
        }));
        controller.connect_leave(clone!(@weak info_label => move |_| {
            info_label.remove_css_class("warning_hover");
            info_label.set_label("");
        }));

        reboot_button.connect_clicked(move |this| {
            this.add_css_class("warning_button_clicked");
            println!("Shutting down system");
            Command::new("systemctl").arg("reboot").output().expect("unable to shutdown");
        });
    }
    fn setup_brightness_button(&self) {
        let controller = EventControllerMotion::new();
        let info_label = self.imp().info_label.get();
        let brightness_button = self.imp().brightness_button.get();

        brightness_button.add_css_class("high");
        // adding a widget controller to show info on hover
        brightness_button.add_controller(&controller);
        controller.connect_enter(clone!(@weak info_label => move |_,_,_| {
            info_label.set_label("Screen Brightness");
        }));
        controller.connect_leave(clone!(@weak info_label => move |_| {
            info_label.set_label("");
        }));
        brightness_button.connect_clicked(move |_| {
            println!("[WIP]");
            info_label.set_label("[WIP]");
        });
    }
}