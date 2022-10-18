use glib::Object;
use gtk::glib;

mod imp;

glib::wrapper! {
    pub struct LockSwayToggle(ObjectSubclass<imp::LockSwayToggle>)
        @extends gtk::Button, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable,
                    gtk::Buildable, gtk::ConstraintTarget;
}

impl LockSwayToggle {
    pub fn new() -> Self {
        Object::new(&[]).expect("Failed to create `SwayLockToggle`.")
    }
}

impl Default for LockSwayToggle {
    fn default() -> Self {
        Self::new()
    }
}

glib::wrapper! {
    pub struct ExitSway(ObjectSubclass<imp::ExitSway>)
        @extends gtk::Button, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable,
                    gtk::Buildable, gtk::ConstraintTarget;
}

impl ExitSway {
    pub fn new() -> Self {
        Object::new(&[]).expect("Failed to create `ExitSway button`.")
    }
}

impl Default for ExitSway {
    fn default() -> Self {
        Self::new()
    }
}