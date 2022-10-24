fn main() {
    glib_build_tools::compile_resources(
        "gtk-ui",
        "gtk-ui/gresource.xml",
        "swaymenu.gresource",
    );
}
