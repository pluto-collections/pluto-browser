use super::{
    browser::Browser,
    searchbar::{get_url, SearchBar, SearchType},
};
use gdk::gdk_pixbuf::{InterpType, Pixbuf};
use gtk::prelude::{EntryExt, HeaderBarExt, WidgetExt};
use std::{io::Cursor, rc::Rc};
use webkit2gtk::WebViewExt;

pub struct Headerbar {
    headerbar: gtk::HeaderBar,
    searchbar: Rc<SearchBar>,
}

impl Headerbar {
    pub fn new(css_provider: Rc<gtk::CssProvider>) -> Self {
        let headerbar = gtk::HeaderBar::new();
        let searchbar = Rc::new(SearchBar::new(css_provider));
        headerbar.set_show_close_button(true);

        headerbar.set_custom_title(Some(searchbar.get_widget()));

        let logo_bytes: &[u8] = include_bytes!("../../assets/pluto.png");
        let reader = Cursor::new(logo_bytes);
        let pixbuf = Pixbuf::from_read(reader).unwrap();
        let pixbuf = pixbuf.scale_simple(20, 20, InterpType::Bilinear).unwrap();
        let image = gtk::Image::from_pixbuf(Some(&pixbuf));
        image.set_size_request(30, 30);
        headerbar.pack_start(&image);

        Headerbar {
            headerbar,
            searchbar,
        }
    }

    pub fn get_widget(&self) -> &gtk::HeaderBar {
        &self.headerbar
    }

    pub fn connect_searchbar_with_browser(&self, browser: Rc<Browser>) {
        let browser_copy = browser.clone();
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

        let searchbar = Rc::clone(&self.searchbar);
        browser.get_widget().connect_uri_notify(move |webview| {
            let uri = webview.uri().unwrap();
            searchbar.get_widget().set_text(&uri);
        });
    }
}
