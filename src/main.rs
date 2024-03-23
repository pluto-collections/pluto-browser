extern crate gio;
extern crate glib;
extern crate gtk;
extern crate webkit2gtk;

use gtk::prelude::*;

mod components;
mod gui;
#[cfg(test)]
mod tests;
mod utils;

fn main() -> std::io::Result<()> {
    let application = gtk::Application::new(
        Some("com.github.pluto_browser"),
        gio::ApplicationFlags::empty(),
    );
    application.connect_activate(|app| {
        // We build the application UI.
        gui::build_ui(app);
    });
    application.run();

    Ok(())
}
