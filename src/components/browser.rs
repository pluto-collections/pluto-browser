use gtk::{
    prelude::{ButtonExt, ContainerExt, StackExt, StackSwitcherExt, WidgetExt},
    Stack, StackSwitcher,
};
use std::sync::Arc;
use uuid::Uuid;
use webkit2gtk::{SettingsExt, WebView, WebViewExt};

use super::headerbar::Headerbar;

#[derive(Clone)]
pub struct Browser {
    pub headerbar: Headerbar,
    pub browser: gtk::Box,
}

impl Browser {
    pub fn new() -> Self {
        let stack = Arc::new(Stack::new());
        let stack_switcher = Arc::new(StackSwitcher::new());

        Self::new_webview(Arc::clone(&stack), Arc::clone(&stack_switcher));

        stack_switcher.set_stack(Some(&*stack));

        let css_provider = Arc::new(gtk::CssProvider::new());
        let headerbar = Headerbar::new(css_provider);
        headerbar.connect_with_browser(Arc::clone(&stack_switcher));

        let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
        vbox.set_expand(true);

        vbox.add(&*stack_switcher);
        vbox.add(&*stack);

        let stack_clone = Arc::clone(&stack);
        let stack_switcher_clone = Arc::clone(&stack_switcher);
        headerbar.add_button.button.connect_clicked(move |_| {
            let stack_clone = Arc::clone(&stack_clone);
            let stack_switcher_clone = Arc::clone(&stack_switcher_clone);
            Self::new_webview(stack_clone, stack_switcher_clone);
        });

        Browser {
            headerbar,
            browser: vbox,
        }
    }

    fn new_webview(stack: Arc<Stack>, stack_switcher: Arc<StackSwitcher>) {
        // create a webcontext
        let web_context = webkit2gtk::WebContext::default().unwrap();

        // Create a new WebView
        let webview = WebView::with_context(&web_context);
        let settings = WebViewExt::settings(&webview).unwrap();
        settings.set_enable_developer_extras(true);
        let homepage_html = include_str!("../pages/homepage.html");

        // by default, load homepage
        webview.load_html(homepage_html, None);

        let web_settings = WebViewExt::settings(&webview).unwrap();
        web_settings.set_enable_developer_extras(true);
        // Load initial URL
        webview.set_expand(true);

        stack.add_titled(&webview, Uuid::new_v4().to_string().as_str(), "New Tab");
        stack.show_all();

        stack_switcher.set_stack(Some(&*stack));
        stack_switcher.show_all();
    }
}
