extern crate gio;
extern crate glib;
extern crate gtk;
extern crate webkit2gtk;

use gtk::{prelude::*, Window, WindowType};
use webkit2gtk::{SettingsExt, WebContext, WebView, WebViewExt};

fn main() {
    gtk::init().unwrap();

    let window = Window::new(WindowType::Toplevel);
    let context = WebContext::default().unwrap();
    // context.set_web_extensions_directory("../webkit2gtk-webextension-rs/example/target/debug/");
    #[cfg(not(feature = "v2_6"))]
    let webview = WebView::with_context(&context);
    webview.load_uri("https://crates.io/");
    window.add(&webview);

    let settings = WebViewExt::settings(&webview).unwrap();
    settings.set_enable_developer_extras(true);

    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        glib::Propagation::Proceed
    });

    gtk::main();
}
