use gtk::{glib};
use gtk::subclass::prelude::*;

// Object holding the state
#[derive(Default)]
pub struct LockButton;

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for LockButton {
    const NAME: &'static str = "CustomLockButton";
    type Type = super::LockButton;
    type ParentType = gtk::Button;
}

// Trait shared by all GObjects
impl ObjectImpl for LockButton {}

// Trait shared by all widgets
impl WidgetImpl for LockButton {}

// Trait shared by all buttons
impl ButtonImpl for LockButton {}
