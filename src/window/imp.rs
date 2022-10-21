#![allow(unused_imports)]

use std::cell::Cell;
use glib::subclass::InitializingObject;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, Label, Button, Frame, Viewport, Grid, AspectFrame, EventController, EventControllerMotion};
use std::process::Command;
use std::ptr::NonNull;
use std::thread;
use std::time::Duration;
use gio::glib::{clone, Type};
use gio::glib::subclass::TypeData;
use gtk::ffi::{GtkEventController, GtkEventControllerMotion, GtkEventControllerMotionClass};

use crate::custom_widgets::{LockButton};

// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/gtk_rs/example/MainWindow.ui")]
pub struct Window {
    #[template_child]
    pub start_win_vm: TemplateChild<Button>,
    #[template_child]
    pub lock_button: TemplateChild<LockButton>,
    #[template_child]
    pub lock_label: TemplateChild<Label>,
    pub lock_state: Cell<i32>,
    #[template_child]
    pub exit_button: TemplateChild<Button>,
    #[template_child]
    pub exit_button_label: TemplateChild<Label>,
    #[template_child]
    pub shutdown_button: TemplateChild<Button>,
    #[template_child]
    pub reboot_button: TemplateChild<Button>,
    #[template_child]
    pub info_label: TemplateChild<Label>,
    #[template_child]
    pub brightness_button: TemplateChild<Button>,
}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for Window {
    // `NAME` needs to match `class` attribute of template
    const NAME: &'static str = "MyGtkAppWindow";
    type Type = super::Window;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();
        LockButton::ensure_type();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

// Trait shared by all GObjects
impl ObjectImpl for Window {
    fn constructed(&self, obj: &Self::Type) {
        // Call "constructed" on parent
        self.parent_constructed(obj);
        obj.setup_lock_button();
        obj.setup_exit_button();
        obj.setup_win_vm_button();
        obj.setup_shutdown_button();
        obj.setup_reboot_button();
        obj.setup_brightness_button();

        self.exit_button.add_css_class("warning_button");
        self.shutdown_button.add_css_class("warning_button");
        self.reboot_button.add_css_class("warning_button");
    }
}
#[gtk::template_callbacks]
impl Window {
    #[template_callback]
    fn handle_button_clicked(&self, button: &LockButton) {
        // Increase state by parameter and save state
        self.lock_state.set((self.lock_state.get() + 1) % 2);

        // Update label with new state
        if self.lock_state.get() == 1 {
            button.set_css_classes(&["lock-closed"]);
            self.lock_label.set_label("");
            self.info_label.set_label("Screen Timeout\nON");
            Command::new("sh").arg("scripts/swayidle.sh").spawn().expect("failed to execute process")
        } else {
            button.set_css_classes(&["lock-open"]);
            self.lock_label.set_label("");
            self.info_label.set_label("Screen Timeout\nOFF");
            Command::new("killall").arg("swayidle").spawn().expect("failed to execute process")
        };
    }
}


// Trait shared by all widgets
impl WidgetImpl for Window {}

// Trait shared by all windows
impl WindowImpl for Window {}

// Trait shared by all application windows
impl ApplicationWindowImpl for Window {}