mod imp;

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use glib::{Object, clone, timeout_future_seconds};
use gtk::subclass::prelude::*;
use gtk::prelude::*;
use gtk::{gio, glib, Application, EventControllerMotion, Button};
use std::process::Command;
use debug_print::debug_println;
use gio::glib::MainContext;


glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Window {
    pub fn new(app: &Application) -> Self {
        // Create new window
        Object::new::<Window>(&[("application", app)])
    }
    // implement lock button
    fn setup_lock_button(&self) {
        let controller = EventControllerMotion::new();
        let info_label = self.imp().info_label.get();
        let lock_button = self.imp().lock_button.get();
        let lock_label = self.imp().lock_label.get();

        lock_label.set_label("");
        lock_button.set_css_classes(&["lock-closed"]);
        self.imp().lock_state.set(1);

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
            Command::new("systemctl").arg("poweroff").output().expect("unable to shutdown");
        });
    }
    fn setup_win_vm_button(&self) {
        let controller = EventControllerMotion::new();
        let info_label = self.imp().info_label.get();
        let win_button = self.imp().start_win_vm.get();
        let spinner = self.imp().spinner.get();

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
            info_label.hide();
            let main_context = MainContext::default();
            main_context.spawn_local(clone!(@weak spinner, @weak info_label => async move {
                spinner.start();

                let mut win_start_command = Command::new("sh").arg("/home/p3rtang/IdeaProjects/swaymenu/scripts/start_vm.sh").spawn().expect("unable to start win11 vm");
                // wait for the command to exit in a non blocking way
                while win_start_command.try_wait().unwrap() == None {
                    timeout_future_seconds(1).await;
                }
                timeout_future_seconds(8).await;
                Command::new("ddcutil").arg("setvcp").arg("60").arg("0x0f").output().unwrap();
                spinner.stop();
                info_label.show();
            }));
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
        fn brightness_set_css(button: &Button, brightness: f32) {
            if brightness <= 40.0 {
                button.set_css_classes(&["low"]);
            } else {
                button.set_css_classes(&["high"]);
            }
        }
        fn get_brightness_file_path() -> Option<PathBuf> {
            return match Path::new("/sys/class/backlight").read_dir() {
                Ok(dir) => {
                    for folder in dir {
                        // only advance if the found entry is a dir
                        if folder.as_ref().expect("Could not access file").path().is_dir() {
                            if let Ok(sub_dir) = folder.as_ref().unwrap().path().read_dir() {
                                for file in sub_dir {
                                    if file.expect("Could not access file").file_name() == "brightness" {
                                        debug_println!("[INFO] found brightness folder path");
                                        return Some(folder.unwrap().path())
                                    }
                                }
                            }
                        }
                    }
                    None
                }
                Err(_) => { None }
            }
        }
        let controller = EventControllerMotion::new();
        let info_label = self.imp().info_label.get();
        let brightness_button = self.imp().brightness_button.get();

        let brightness_folder = get_brightness_file_path();
        if let Some(folder) = brightness_folder {
            if !folder.join("brightness").metadata().unwrap().permissions().readonly() {
                debug_println!("[INFO] brightness file correct permissions");
                let max = fs::read_to_string(folder.join("max_brightness"))
                    .expect("Should have been able to read the file").replace('\n', "").parse::<f32>().unwrap();
                let current = fs::read_to_string(folder.join("brightness"))
                    .expect("Should have been able to read the file").replace('\n', "").parse::<f32>().unwrap();
                self.imp().brightness.set((current / max * 100.0).ceil());
                brightness_set_css(&brightness_button, current / max * 100.0);
                brightness_button.add_controller(&controller);
                controller.connect_enter(clone!(@weak info_label, @weak self as window => move |_,_,_| {
                    info_label.set_label(&format!("Brightness {}%", window.imp().brightness.get().ceil()))
                }));
                controller.connect_leave(clone!(@weak info_label => move |_| {
                    info_label.set_label("");
                }));
                brightness_button.connect_clicked(clone!(@weak info_label, @weak brightness_button, @weak self as window => move |_| {
                    if window.imp().brightness.get() < 20.0 {
                        window.imp().brightness.set(100.0)
                    } else if window.imp().brightness.get() < 21.0 {
                        window.imp().brightness.set(1.0)
                    } else {
                        window.imp().brightness.set(window.imp().brightness.get() - 20.0)
                    }
                    info_label.set_label(&format!("Brightness {}%", window.imp().brightness.get().ceil()));
                    brightness_set_css(&brightness_button, window.imp().brightness.get());

                    let mut f = fs::OpenOptions::new().write(true).open(folder.join("brightness")).unwrap();
                    f.write_all(format!("{}", (window.imp().brightness.get() / 100.0 * max) as i32).as_bytes()).unwrap();
                    f.flush().unwrap();

                }));
                return
            }
        }
        let brightness_vec = Command::new("ddcutil").arg("getvcp").arg("10").output().unwrap();

        self.imp().brightness.set(100.0);
        if let Ok(out)= String::from_utf8(brightness_vec.stdout) {
            let out_vec = out.split_ascii_whitespace().collect::<Vec<&str>>();
            if let Ok(brightness_float) = out_vec[8].replace(',', "").parse::<f32>() {
                self.imp().brightness.set(brightness_float);
                brightness_set_css(&brightness_button, self.imp().brightness.get());
                // adding a widget controller to show info on hover
                brightness_button.add_controller(&controller);
                controller.connect_enter(clone!(@weak info_label, @weak self as window => move |_,_,_| {
                    info_label.set_label(&format!("Brightness {}%", window.imp().brightness.get()))
                }));
                controller.connect_leave(clone!(@weak info_label => move |_| {
                    info_label.set_label("");
                }));
                brightness_button.connect_clicked(clone!(@weak info_label, @weak brightness_button, @weak self as window => move |_| {
                    if window.imp().brightness.get() < 20.0 {
                        window.imp().brightness.set(100.0)
                    } else if window.imp().brightness.get() < 21.0 {
                        window.imp().brightness.set(1.0)
                    } else {
                        window.imp().brightness.set(window.imp().brightness.get() - 20.0)
                    }
                    info_label.set_label(&format!("Brightness {}%", window.imp().brightness.get()));
                    brightness_set_css(&brightness_button, window.imp().brightness.get());

                    Command::new("ddcutil").arg("setvcp").arg("10").arg(format!("{}", window.imp().brightness.get())).output().unwrap();
                }));
            } else {
                brightness_button.set_sensitive(false);
            }
        }
    }
    fn setup_sleep_button(&self) {
        let sleep_button = self.imp().sleep_button.get();
        let info_label = self.imp().info_label.get();
        let controller = EventControllerMotion::new();

        sleep_button.add_controller(&controller);
        controller.connect_enter(clone!(@weak info_label, @weak self as window => move |_,_,_| {
            info_label.set_label("Suspend");
            info_label.set_css_classes(&["sleep"]);
        }));
        controller.connect_leave(clone!(@weak info_label, @weak self as window => move |_| {
            info_label.set_label("");
            info_label.remove_css_class("sleep");
        }));

        sleep_button.connect_clicked(move |_| {
            Command::new("systemctl").arg("suspend").output().unwrap();
        });
    }
    fn setup_volume_button(&self) {
        let volume_button = self.imp().sound_level_button.get();
        let info_label = self.imp().info_label.get();
        let controller = EventControllerMotion::new();

        volume_button.add_controller(&controller);
        controller.connect_enter(clone!(@weak info_label, @weak self as window => move |_,_,_| {
            info_label.set_label("Volume");
        }));
        controller.connect_leave(clone!(@weak info_label, @weak self as window => move |_| {
            info_label.set_label("");
        }));
    }
}
