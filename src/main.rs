extern crate gio;
extern crate glib;
extern crate gtk;
extern crate webkit2gtk;

use gtk::prelude::*;
use webkit2gtk::{SettingsExt, WebContext, WebView, WebViewExt};

fn build_ui(application: &gtk::Application) -> std::io::Result<()> {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("Kass Browser");
    window.set_position(gtk::WindowPosition::Center);

    let button = gtk::Button::with_label("Click me!");
    let label = gtk::Label::new(Some("0"));
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);

    if let (Some(button_obj), Some(label_obj)) = (button.accessible(), label.accessible()) {
        // We set the description
        button_obj.set_description("Button to increase label value");

        // Then we setup the relation saying that the label is linked to the button.
        let relation_set = label_obj
            .ref_relation_set()
            .expect("Failed to get relation for label");
        let relation = atk::Relation::new(&[button_obj], atk::RelationType::LabelFor);

        relation_set.add(&relation);
    }

    let context = WebContext::default().unwrap();
    let webview = WebView::with_context(&context);
    let uri = "https://crates.io/";
    webview.load_uri(uri);
    webview.set_vexpand(true);

    let settings = WebViewExt::settings(&webview).unwrap();
    settings.set_enable_developer_extras(true);

    let textbox = gtk::Entry::new();
    textbox.set_text(uri);
    textbox.set_hexpand(true);

    let loading_label = gtk::Label::new(None);

    vbox.add(&button);
    vbox.add(&label);
    vbox.add(&textbox);
    vbox.add(&loading_label);
    vbox.add(&webview);

    window.add(&vbox);

    textbox.connect_activate(move |entry| {
        let text = entry.text();
        webview.load_uri(&text);
        // await till the page is loaded
        while webview.is_loading() {
            gtk::main_iteration();
            loading_label.set_text("Loading...");
        }

        // reset the loading label to None
        loading_label.set_text("");
        entry.set_text(&webview.uri().unwrap());
    });

    window.show_all();

    Ok(())
}

fn main() -> std::io::Result<()> {
    let application = gtk::Application::new(
        Some("com.github.accessibility"),
        gio::ApplicationFlags::empty(),
    );
    application.connect_activate(|app| {
        // We build the application UI.
        build_ui(app).unwrap();
    });
    application.run();

    Ok(())
}
