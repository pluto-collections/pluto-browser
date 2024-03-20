use glib::clone;
use gtk::prelude::*;
use gtk_sys::GTK_STYLE_CLASS_TITLE;
use std::{rc::Rc, str};

pub struct Headerbar {
    pub headerbar: gtk::Box,
    pub minimize_btn: gtk::Button,
    pub maximize_btn: gtk::Button,
    pub close_btn: gtk::Button,
}

impl Headerbar {
    pub fn new(window: Rc<gtk::ApplicationWindow>) -> Self {
        let headerbar = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        headerbar.set_size_request(10, 10);
        headerbar.set_hexpand(true);
        headerbar.set_opacity(1.0);

        // load stylesheet
        let provider = gtk::CssProvider::new();
        let style_file = include_str!("../styles/style.css");
        provider.load_from_data(style_file.as_bytes()).unwrap();

        let title = gtk::Label::new(Some("Hello World"));
        let title_class = match str::from_utf8(GTK_STYLE_CLASS_TITLE) {
            Ok(s) => s,
            Err(e) => panic!("Invalid UTF-8: {}", e), // Or handle the error appropriately
        };
        let title_class = title_class.to_string();
        let title_class = title_class.trim();

        // load css file
        title
            .style_context()
            .add_provider(&provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
        title.style_context().add_class("test");
        let filtered_class = title_class.replace('\0', ""); // Remove null byte
        title.style_context().add_class(&filtered_class);
        // headerbar
        //     .style_context()
        //     .add_provider(&provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
        let action_bar = gtk::ActionBar::new();
        action_bar.style_context().add_class("actionbar");
        headerbar.pack_end(&action_bar, false, false, 0);

        let minimize_btn =
            gtk::Button::from_icon_name(Some("window-minimize"), gtk::IconSize::Menu);
        minimize_btn.set_valign(gtk::Align::Center);

        minimize_btn.style_context().add_class("titlebutton");
        minimize_btn.style_context().add_class("minimize");
        action_bar.pack_start(&minimize_btn);

        let maximize_btn =
            gtk::Button::from_icon_name(Some("window-maximize"), gtk::IconSize::Menu);
        maximize_btn.set_valign(gtk::Align::Center);

        maximize_btn.style_context().add_class("titlebutton");
        maximize_btn.style_context().add_class("maximize");
        action_bar.set_center_widget(Some(&maximize_btn));

        let close_btn = gtk::Button::from_icon_name(Some("window-close"), gtk::IconSize::Menu);
        close_btn.set_valign(gtk::Align::Center);

        close_btn.style_context().add_class("titlebutton");
        close_btn.style_context().add_class("close");
        action_bar.pack_end(&close_btn);

        headerbar.style_context().add_class("headerbar");
        headerbar.add(&title);

        maximize_btn.connect_clicked(clone!(@weak window => move |_| {
            if window.is_maximized() {
                window.unmaximize();
            } else {
                window.maximize();
            }
        }));

        minimize_btn.connect_clicked(clone!(@weak window => move |_| {
            window.iconify();
        }));

        close_btn.connect_clicked(clone!(@weak window => move |_| {
            window.close();
        }));

        Headerbar {
            headerbar,
            minimize_btn,
            maximize_btn,
            close_btn,
        }
    }

    pub fn get_widget(&self) -> &gtk::Box {
        &self.headerbar
    }
}
