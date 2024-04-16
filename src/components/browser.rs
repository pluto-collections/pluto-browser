use gtk::prelude::WidgetExt;
use webkit2gtk::{SettingsExt, WebView, WebViewExt};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AboutType {
    Home,
    Blank,
    Unknown,
}

#[derive(Clone)]
pub struct Browser {
    widget: WebView,
}

impl Browser {
    pub fn new() -> Self {
        // Create a new WebView
        let webview = WebView::new();
        let settings = WebViewExt::settings(&webview).unwrap();
        settings.set_enable_developer_extras(true);
        let homepage_html = include_str!("../pages/homepage.html");

        // by default, load homepage
        webview.load_html(homepage_html, None);

        let web_settings = WebViewExt::settings(&webview).unwrap();
        web_settings.set_enable_developer_extras(true);
        // Load initial URL
        webview.set_expand(true);

        Browser { widget: webview }
    }

    pub fn get_widget(&self) -> &WebView {
        &self.widget
    }

    pub fn update_uri(&self, uri: &str) {
        self.widget.load_uri(uri);
    }

    fn get_about_type(uri: &String) -> AboutType {
        match uri.as_str() {
            "about:home" => AboutType::Home,
            "about:blank" => AboutType::Blank,
            _ => AboutType::Unknown,
        }
    }

    pub fn load_about_pages(&self, uri: &String) {
        let about_type = Self::get_about_type(uri);
        match about_type {
            AboutType::Home => {
                let homepage_html = include_str!("../pages/homepage.html");
                self.widget.load_html(homepage_html, None);
            }
            AboutType::Blank => {
                let blank_html = include_str!("../pages/blank.html");
                self.widget.load_html(blank_html, None);
            }
            AboutType::Unknown => {
                let unknown_html = include_str!("../pages/unknown.html");
                self.widget.load_html(unknown_html, None);
            }
        }
    }
}
