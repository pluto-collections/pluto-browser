use std::sync::{Arc, Mutex};

use crate::components::{browser, headerbar};
use gtk::prelude::*;
use webkit2gtk::WebViewExt;

pub fn build_ui(application: &gtk::Application) {
    //=========================================================================
    // BUILD UI
    //=========================================================================
    let window = Arc::new(gtk::ApplicationWindow::new(application));
    window.set_default_size(800, 600);

    let css_provider = Arc::new(gtk::CssProvider::new());
    let css_file = include_str!("./styles/style.css");
    css_provider.load_from_data(css_file.as_bytes()).unwrap();

    let headerbar = headerbar::Headerbar::new(Arc::clone(&css_provider));

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    window.add(&vbox);
    vbox.set_expand(true);

    let browser = Arc::new(Mutex::new(browser::Browser::default()));

    //=========================================================================
    // ADD WIDGETS
    //=========================================================================
    window.set_titlebar(Some(headerbar.get_widget()));

    vbox.add(browser.lock().unwrap().get_current().get_widget());

    // Show all widgets
    window.set_title("Pluto Browser");
    window.show_all();
    headerbar.connect_searchbar_with_browser(Arc::clone(&browser.lock().unwrap().get_current()));
    headerbar.connect_add_button_with_browser(Arc::clone(&browser), vbox);

    //=========================================================================
    // CONNECT SIGNALS
    //=========================================================================
    browser
        .lock()
        .unwrap()
        .get_current()
        .get_widget()
        .connect_load_changed(move |webview, _| {
            if let Some(title) = webview.title() {
                window.set_title(&format!("{} - Pluto Browser", title));
            }
        });
}
