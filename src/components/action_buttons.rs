use super::{browser::SingleWebView, button};
use glib::clone;
use gtk::prelude::{BoxExt, ButtonExt};
use std::{cell::Cell, sync::Arc};
use webkit2gtk::{LoadEvent, WebViewExt};

#[derive(Clone)]
pub struct ActionButtons {
    container: Arc<gtk::Box>,
    previous_button: Arc<button::WebViewButton>,
    next_button: Arc<button::WebViewButton>,
    refresh_button: Arc<button::WebViewButton>,
    home_button: Arc<button::WebViewButton>,
    is_refreshing: Arc<Cell<bool>>,
}

impl ActionButtons {
    pub fn new(_css_provider: Arc<gtk::CssProvider>) -> Self {
        let previous_button = Arc::new(button::WebViewButton::new(Some("go-previous")));
        let next_button = Arc::new(button::WebViewButton::new(Some("go-next")));
        let refresh_button = Arc::new(button::WebViewButton::new(Some("view-refresh")));
        let home_button = Arc::new(button::WebViewButton::new(Some("go-home")));

        let hbox = Arc::new(gtk::Box::new(gtk::Orientation::Horizontal, 0));
        hbox.pack_start(&previous_button.button, false, false, 4);
        hbox.pack_start(&next_button.button, false, false, 4);
        hbox.pack_start(&refresh_button.button, false, false, 4);
        hbox.pack_start(&home_button.button, false, false, 4);

        ActionButtons {
            is_refreshing: Arc::new(Cell::new(false)),
            previous_button,
            next_button,
            refresh_button,
            home_button,
            container: hbox,
        }
    }

    pub fn get_widget(&self) -> Arc<gtk::Box> {
        Arc::clone(&self.container)
    }
    pub fn connect_action_with_browser(&self, browser: Arc<SingleWebView>) {
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

        let is_refreshing = Arc::clone(&self.is_refreshing);
        self.refresh_button
            .button
            .connect_clicked(clone!(@strong browser => move |_| {
                if is_refreshing.get() {
                    browser.get_widget().stop_loading();
                } else {
                    browser.get_widget().reload();
                }
            }));

        self.home_button
            .button
            .connect_clicked(clone!(@strong browser => move |_| {
                browser.load_about_pages(&"about:home".to_string());
            }));

        let refresh_button = Arc::clone(&self.refresh_button);
        let is_refreshing = Arc::clone(&self.is_refreshing);
        browser.get_widget().connect_load_changed(
            move |_, load_event: LoadEvent| match load_event {
                LoadEvent::Started => {
                    refresh_button.set_icon_name("process-stop");
                    is_refreshing.set(true);
                }
                LoadEvent::Finished => {
                    refresh_button.set_icon_name("view-refresh");
                    is_refreshing.set(false);
                }
                _ => {}
            },
        );
    }
}
