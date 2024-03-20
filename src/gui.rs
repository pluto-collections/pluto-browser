use crate::components::{
    browser, button,
    searchbar::{self, get_url},
};
use glib::clone;
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
    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    window.add(&vbox);
    vbox.set_expand(true);

    let browser = browser::Browser::new();
    let searchbar = searchbar::SearchBar::new();
    let previous_button = button::WebViewButton::new(Some("go-previous"));
    let next_button = button::WebViewButton::new(Some("go-next"));
    let refresh_button = button::WebViewButton::new(Some("view-refresh"));

    let button_container = gtk::Box::new(gtk::Orientation::Horizontal, 10);

    vbox.add(&hbox);
    vbox.add(browser.get_widget());

    hbox.pack_start(&button_container, false, false, 20);
    hbox.pack_start(&previous_button.button, false, false, 4);
    hbox.pack_start(&next_button.button, false, false, 4);
    hbox.pack_start(&refresh_button.button, false, false, 4);
    hbox.pack_start(searchbar.get_widget(), true, true, 10);

    // Connect the button to the browser
    previous_button
        .button
        .connect_clicked(clone!(@strong browser => move |_| {
            browser.get_widget().go_back();
        }));

    // Connect the button to the browser
    next_button
        .button
        .connect_clicked(clone!(@strong browser => move |_| {
            browser.get_widget().go_forward();
        }));

    refresh_button
        .button
        .connect_clicked(clone!(@strong browser => move |_| {
            browser.get_widget().reload();
        }));

    // Connect the searchbar to the browser
    searchbar
        .get_widget()
        .connect_activate(clone!(@strong browser =>  move |entry| {
            let uri = entry.text();
            let uri = get_url(&uri.to_string());
            browser.clone().update_uri(&uri);
        }));

    // update the uri when the webview changes
    browser
        .get_widget()
        .connect_uri_notify(clone!(@strong browser  => move |webview| {
            let uri = webview.uri().unwrap();
            searchbar.get_widget().set_text(&uri);
        }));

    // Show all widgets
    window.show_all();
}
