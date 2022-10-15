use gtk::gio;

fn main() {
    gio::compile_resources(
        "gtk-ui",
        "gtk-ui/gresource.xml",
        "composite_templates_1.gresource",
    );
}
