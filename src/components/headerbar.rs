use super::{
    action_buttons,
    button::WebViewButton,
    searchbar::{get_url, SearchBar, SearchType},
};
use gdk::gdk_pixbuf::{InterpType, Pixbuf};
use glib::Cast;
use gtk::{
    prelude::{ContainerExt, EntryExt, HeaderBarExt, StackExt, StackSwitcherExt, WidgetExt},
    StackSwitcher,
};
use std::{io::Cursor, sync::Arc};
use webkit2gtk::{WebView, WebViewExt};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AboutType {
    Home,
    Blank,
    Unknown,
}

#[derive(Clone)]
pub struct Headerbar {
    headerbar: gtk::HeaderBar,
    searchbar: Arc<SearchBar>,
    action_btns: action_buttons::ActionButtons,
    pub add_button: WebViewButton,
}

impl Headerbar {
    pub fn new() -> Self {
        let headerbar = gtk::HeaderBar::new();
        let searchbar = Arc::new(SearchBar::new());
        let action_btns = action_buttons::ActionButtons::new();
        headerbar.set_show_close_button(true);

        headerbar.set_custom_title(Some(searchbar.get_widget()));

        let logo_bytes: &[u8] = include_bytes!("../../assets/pluto.png");
        let reader = Cursor::new(logo_bytes);
        let pixbuf = Pixbuf::from_read(reader).unwrap();
        let pixbuf = pixbuf.scale_simple(20, 20, InterpType::Bilinear).unwrap();
        let image = gtk::Image::from_pixbuf(Some(&pixbuf));
        image.set_size_request(30, 30);
        headerbar.pack_start(&image);

        headerbar.add(&*action_btns.get_widget());

        let add_button = WebViewButton::new(Some("list-add"));

        headerbar.add(&add_button.button);

        Headerbar {
            headerbar,
            searchbar,
            add_button,
            action_btns,
        }
    }

    pub fn get_widget(&self) -> &gtk::HeaderBar {
        &self.headerbar
    }

    pub fn connect_with_browser(&self, stack_switcher: Arc<StackSwitcher>) {
        // connect searchbar with browser
        let stack_switcher_clone = Arc::clone(&stack_switcher);
        self.searchbar.get_widget().connect_activate(move |entry| {
            let uri = entry.text();
            if uri.trim().is_empty() {
                return;
            }
            let search_type = crate::utils::get_search_type(&uri.to_string());

            if search_type == SearchType::About {
                load_about_pages(&uri.to_string(), Arc::clone(&stack_switcher_clone));
                return;
            }

            let uri = get_url(&uri.to_string(), search_type);
            update_uri(&uri, Arc::clone(&stack_switcher_clone));
        });

        // connect action buttons with browser
        self.action_btns
            .connect_action_with_browser(Arc::clone(&stack_switcher));
    }
}

pub fn load_about_pages(uri: &String, stack_switcher: Arc<StackSwitcher>) {
    let about_type = get_about_type(uri);

    // Assume `stack` is a gtk::Stack instance stored in the struct
    let current_widget = stack_switcher.stack().and_then(|stack| {
        // Get the currently visible child from the stack
        stack.visible_child().and_then(|child| {
            // Downcast the child to WebView
            child.dynamic_cast::<WebView>().ok()
        })
    });

    if let Some(webview) = current_widget {
        match about_type {
            AboutType::Home => {
                let homepage_html = include_str!("../pages/homepage.html");
                webview.load_html(homepage_html, None);
            }
            AboutType::Blank => {
                let blank_html = include_str!("../pages/blank.html");
                webview.load_html(blank_html, None);
            }
            AboutType::Unknown => {
                let unknown_html = include_str!("../pages/unknown.html");
                webview.load_html(unknown_html, None);
            }
        }
    }
}
fn get_about_type(uri: &String) -> AboutType {
    match uri.as_str() {
        "about:home" => AboutType::Home,
        "about:blank" => AboutType::Blank,
        _ => AboutType::Unknown,
    }
}

pub fn update_uri(uri: &str, stack_switcher: Arc<StackSwitcher>) {
    // Assume `stack` is a gtk::Stack instance stored in the struct
    let current_widget = stack_switcher.stack().and_then(|stack| {
        // Get the currently visible child from the stack
        stack.visible_child().and_then(|child| {
            // Downcast the child to WebView
            child.dynamic_cast::<WebView>().ok()
        })
    });

    if let Some(current_widget) = current_widget {
        current_widget.load_uri(uri);
    }
}
