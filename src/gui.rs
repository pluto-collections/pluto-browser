use crate::components::{browser, searchbar};
use gtk::prelude::*;

pub fn build_ui(application: &gtk::Application) {
    // Create a window
    let window = gtk::ApplicationWindow::new(application);
    window.set_title("Pluto Browser");
    window.set_default_size(800, 600);

    // Connect destroy event to quit the application
    window.connect_destroy(|_| {
        gtk::main_quit();
    });

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    window.add(&vbox);
    vbox.set_expand(true);

    let browser = browser::Browser::new();
    let searchbar = searchbar::SearchBar::new();
    vbox.add(searchbar.get_widget());

    vbox.add(browser.get_widget());

    // Connect the searchbar to the browser
    searchbar.get_widget().connect_activate(move |entry| {
        let uri = entry.text();
        browser.update_uri(&uri);
    });

    // Show all widgets
    window.show_all();
}
