use super::{
    action_buttons,
    browser::{Browser, SingleWebView},
    button,
    searchbar::{get_url, SearchBar, SearchType},
};
use gdk::gdk_pixbuf::{InterpType, Pixbuf};
use gtk::prelude::{ButtonExt, ContainerExt, EntryExt, HeaderBarExt, WidgetExt};
use std::{
    io::Cursor,
    sync::{Arc, Mutex},
};
use webkit2gtk::WebViewExt;

pub struct Headerbar {
    headerbar: gtk::HeaderBar,
    searchbar: Arc<SearchBar>,
    action_btn: action_buttons::ActionButtons,
    add_button: Arc<button::WebViewButton>,
}

impl Headerbar {
    pub fn new(css_provider: Arc<gtk::CssProvider>) -> Self {
        let css_provider_copy = Arc::clone(&css_provider);
        let headerbar = gtk::HeaderBar::new();
        let searchbar = Arc::new(SearchBar::new(css_provider));
        let action_btn = action_buttons::ActionButtons::new(css_provider_copy);
        let add_button = Arc::new(button::WebViewButton::new(Some("list-add")));
        headerbar.set_show_close_button(true);

        headerbar.set_custom_title(Some(searchbar.get_widget()));

        let logo_bytes: &[u8] = include_bytes!("../../assets/pluto.png");
        let reader = Cursor::new(logo_bytes);
        let pixbuf = Pixbuf::from_read(reader).unwrap();
        let pixbuf = pixbuf.scale_simple(20, 20, InterpType::Bilinear).unwrap();
        let image = gtk::Image::from_pixbuf(Some(&pixbuf));
        image.set_size_request(30, 30);
        headerbar.pack_start(&image);

        headerbar.add(&*action_btn.get_widget());
        headerbar.add(&add_button.button);
        Headerbar {
            headerbar,
            searchbar,
            action_btn,
            add_button,
        }
    }

    pub fn get_widget(&self) -> &gtk::HeaderBar {
        &self.headerbar
    }

    pub fn connect_searchbar_with_browser(&self, browser: Arc<SingleWebView>) {
        let browser_copy = browser.clone();
        self.action_btn.connect_action_with_browser(browser.clone());
        self.searchbar.get_widget().connect_activate(move |entry| {
            let uri = entry.text();
            if uri.trim().is_empty() {
                return;
            }
            let search_type = crate::utils::get_search_type(&uri.to_string());

            if search_type == SearchType::About {
                browser_copy.load_about_pages(&uri.to_string());
                return;
            }

            let uri = get_url(&uri.to_string(), search_type);
            browser_copy.update_uri(&uri);
        });

        let searchbar = Arc::clone(&self.searchbar);
        browser.get_widget().connect_uri_notify(move |webview| {
            let uri = webview.uri().unwrap();
            searchbar.get_widget().set_text(&uri);
        });
    }

    pub fn connect_add_button_with_browser(&self, browser: Arc<Mutex<Browser>>, vbox: gtk::Box) {
        let browser = Arc::clone(&browser);

        let searchbar = Arc::clone(&self.searchbar);

        self.add_button.button.connect_clicked(move |_| {
            let mut browser = browser.lock().unwrap();
            browser.new_webview();

            let current_webview = browser.get_current();

            // Show all widgets
            vbox.show_all();
            let url_text = current_webview.url_text.lock().unwrap();
            searchbar.get_widget().set_text(url_text.as_str());
        });
    }
}
