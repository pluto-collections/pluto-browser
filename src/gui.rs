use std::rc::Rc;

use crate::components::{browser, headerbar};
use gtk::prelude::*;
use webkit2gtk::WebViewExt;

pub fn build_ui(application: &gtk::Application) {
    //=========================================================================
    // BUILD UI
    //=========================================================================
    let window = Rc::new(gtk::ApplicationWindow::new(application));
    window.set_default_size(800, 600);

    let css_provider = Rc::new(gtk::CssProvider::new());
    let css_file = include_str!("./styles/style.css");
    css_provider.load_from_data(css_file.as_bytes()).unwrap();

    let headerbar = headerbar::Headerbar::new(Rc::clone(&css_provider));

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    window.add(&vbox);
    vbox.set_expand(true);

    let browser = Rc::new(browser::Browser::new());

    //=========================================================================
    // ADD WIDGETS
    //=========================================================================
    window.set_titlebar(Some(headerbar.get_widget()));

    vbox.add(browser.get_widget());

    // Show all widgets
    window.set_title("Pluto Browser");
    window.show_all();
    headerbar.connect_searchbar_with_browser(Rc::clone(&browser));

    //=========================================================================
    // CONNECT SIGNALS
    //=========================================================================
    browser
        .get_widget()
        .connect_load_changed(move |webview, _| {
            if let Some(title) = webview.title() {
                window.set_title(&format!("{} - Pluto Browser", title));
            }
        });
}

