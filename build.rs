fn main() {
    // Попытка найти и настроить Poppler
    pkg_config::Config::new()
        .atleast_version("0.68")
        .probe("poppler-glib")
        .expect("Failed to find poppler-glib");

    // Попытка найти и настроить Cairo
    pkg_config::Config::new()
        .atleast_version("1.14")
        .probe("cairo")
        .expect("Failed to find cairo");

    // Попытка найти и настроить GLib
    pkg_config::Config::new()
        .atleast_version("2.56")
        .probe("glib-2.0")
        .expect("Failed to find glib-2.0");

    // Попытка найти и настроить GObject
    pkg_config::Config::new()
        .atleast_version("2.56")
        .probe("gobject-2.0")
        .expect("Failed to find gobject-2.0");
}
