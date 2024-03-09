use gtk::prelude::WidgetExt;
use webkit2gtk::{WebView, WebViewExt};

pub struct Browser {
    widget: WebView,
}

impl Browser {
    pub fn new() -> Self {
        // Create a new WebView
        let webview = WebView::new();

        // Load initial URL
        // let uri = "https://crates.io";
        // webview.load_uri(&uri);
        webview.set_expand(true);

        Browser { widget: webview }
    }

    pub fn get_widget(&self) -> &WebView {
        &self.widget
    }

    pub fn update_uri(&self, uri: &str) {
        self.widget.load_uri(uri);
    }
}
