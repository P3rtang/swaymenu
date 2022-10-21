use glib::Object;
use gtk::glib;

mod imp;

glib::wrapper! {
    pub struct LockButton(ObjectSubclass<imp::LockButton>)
        @extends gtk::Button, gtk::Widget,
        @implements gtk::Accessible, gtk::Actionable,
                    gtk::Buildable, gtk::ConstraintTarget;
}

impl LockButton {
    pub fn new() -> Self {
        Object::new(&[]).expect("Failed to create `SwayLockToggle`.")
    }
}

impl Default for LockButton {
    fn default() -> Self {
        Self::new()
    }
}
