use std::sync::Arc;

use crate::components::browser;
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

    let browser = Arc::new(browser::Browser::new());

    //=========================================================================
    // ADD WIDGETS
    //=========================================================================
    window.set_titlebar(Some(browser.headerbar.get_widget()));
    window.add(&browser.browser);

    // Show all widgets
    window.set_title("Pluto Browser");
    window.show_all();

    //=========================================================================
    // CONNECT SIGNALS
    //=========================================================================
    // browser
    //     .get_current()
    //     .get_widget()
    //     .connect_load_changed(move |webview, _| {
    //         if let Some(title) = webview.title() {
    //             window.set_title(&format!("{} - Pluto Browser", title));
    //         }
    //     });
}
