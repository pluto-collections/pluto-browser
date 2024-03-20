use crate::components::{
    browser, button,
    searchbar::{self, get_url},
};
use gtk::prelude::*;
use webkit2gtk::WebViewExt;

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
    let browser2 = browser.clone();
    let browser_clone = browser.clone();
    let browser_clone2 = browser.clone();
    let searchbar = searchbar::SearchBar::new();
    let searchbar_clone = searchbar.clone();
    let previous_button = button::WebViewButton::new(Some("go-previous"));
    let next_button = button::WebViewButton::new(Some("go-next"));

    vbox.add(searchbar.get_widget());
    vbox.add(&previous_button.button);
    vbox.add(&next_button.button);
    vbox.add(browser.get_widget());

    // Connect the button to the browser
    previous_button.button.connect_clicked(move |_| {
        browser_clone.get_widget().go_back();
        println!("Going back");
    });

    // Connect the button to the browser
    next_button.button.connect_clicked(move |_| {
        browser_clone2.clone().get_widget().go_forward();
        println!("Going forward");
    });

    // Connect the searchbar to the browser
    searchbar_clone.get_widget().connect_activate(move |entry| {
        let uri = entry.text();
        let uri = get_url(&uri.to_string());
        browser.clone().update_uri(&uri);
    });

    // update the uri when the webview changes
    let browser_clone = browser2.clone();
    browser_clone
        .get_widget()
        .connect_uri_notify(move |webview| {
            let uri = webview.uri().unwrap();
            searchbar.get_widget().set_text(&uri);
        });

    // Show all widgets
    window.show_all();
}
