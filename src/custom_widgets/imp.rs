use gtk::glib;
use gtk::subclass::prelude::*;

// Object holding the state
#[derive(Default)]
pub struct LockSwayToggle;

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for LockSwayToggle {
    const NAME: &'static str = "SwayMenuLockSwayToggle";
    type Type = super::LockSwayToggle;
    type ParentType = gtk::Button;
}

// Trait shared by all GObjects
impl ObjectImpl for LockSwayToggle {}

// Trait shared by all widgets
impl WidgetImpl for LockSwayToggle {}

// Trait shared by all buttons
impl ButtonImpl for LockSwayToggle {}

// Object holding the state
#[derive(Default)]
pub struct ExitSway;

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for ExitSway {
    const NAME: &'static str = "SwayMenuExitSway";
    type Type = super::ExitSway;
    type ParentType = gtk::Button;
}

// Trait shared by all GObjects
impl ObjectImpl for ExitSway {}

// Trait shared by all widgets
impl WidgetImpl for ExitSway {}

// Trait shared by all buttons
impl ButtonImpl for ExitSway {}