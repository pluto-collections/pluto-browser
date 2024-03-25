use super::{browser::Browser, button};
use glib::clone;
use gtk::prelude::{BoxExt, ButtonExt, ContainerExt};
use std::rc::Rc;
use webkit2gtk::WebViewExt;

pub struct ActionButtons {
    previous_button: button::WebViewButton,
    next_button: button::WebViewButton,
    refresh_button: button::WebViewButton,
}

impl ActionButtons {
    pub fn new(_css_provider: Rc<gtk::CssProvider>) -> Self {
        let previous_button = button::WebViewButton::new(Some("go-previous"));
        let next_button = button::WebViewButton::new(Some("go-next"));
        let refresh_button = button::WebViewButton::new(Some("view-refresh"));

        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
        let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);

        vbox.add(&hbox);

        ActionButtons {
            previous_button,
            next_button,
            refresh_button,
        }
    }

    pub fn get_widget(&self) -> gtk::Box {
        let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        hbox.pack_start(&self.previous_button.button, false, false, 4);
        hbox.pack_start(&self.next_button.button, false, false, 4);
        hbox.pack_start(&self.refresh_button.button, false, false, 4);

        hbox
    }
    pub fn connect_action_with_browser(&self, browser: Rc<Browser>) {
        // Connect the button to the browser

        self.previous_button
            .button
            .connect_clicked(clone!(@strong browser => move |_| {
                browser.get_widget().go_back();
            }));
        // Connect the button to the browser
        self.next_button
            .button
            .connect_clicked(clone!(@strong browser => move |_| {
                browser.get_widget().go_forward();
            }));

        self.refresh_button
            .button
            .connect_clicked(clone!(@strong browser => move |_| {
                browser.get_widget().reload();
            }));
    }
}
