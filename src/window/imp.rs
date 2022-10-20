use glib::subclass::InitializingObject;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate, Label, Button};
use std::process::Command;

use crate::custom_widgets::{ExitSway, LockSwayToggle};

// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/org/gtk_rs/example/MainWindow.ui")]
pub struct Window {
    #[template_child]
    pub start_win_vm: TemplateChild<Button>,
    #[template_child]
    pub lock_button: TemplateChild<LockSwayToggle>,
    #[template_child]
    pub button_label: TemplateChild<Label>,
    #[template_child]
    pub exit_button: TemplateChild<ExitSway>,
    #[template_child]
    pub shutdown_button: TemplateChild<Button>
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

        self.exit_button.add_css_class("warning_button");
        self.shutdown_button.add_css_class("warning_button");

        self.shutdown_button.connect_clicked(move |this| {
            this.add_css_class("warning_button_clicked");
            println!("Shutting down system");
            Command::new("systemctl").arg("poweroff").output().expect("unable to shutdown");
        });

        self.start_win_vm.connect_clicked(move |_| {
            println!("startign vm");
            let _start_vm_command = Command::new("sh").current_dir("scripts").arg("start_vm.sh")
                .spawn().expect("unable to start win11 vm");
        });
    }
}

// Trait shared by all widgets
impl WidgetImpl for Window {}

// Trait shared by all windows
impl WindowImpl for Window {}

// Trait shared by all application windows
impl ApplicationWindowImpl for Window {}