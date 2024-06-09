use glib::Cast;
use gtk::{
    prelude::{ButtonExt, ContainerExt, GtkWindowExt, StackExt, StackSwitcherExt, WidgetExt}, ApplicationWindow, Stack, StackSwitcher
};
use std::sync::Arc;
use uuid::Uuid;
use webkit2gtk::{SettingsExt, WebView, WebViewExt};

use super::headerbar::Headerbar;

#[derive(Clone)]
pub struct Browser {
    pub headerbar: Headerbar,
    pub browser: gtk::Box,
    pub stack: Arc<Stack>,
}

impl Browser {
    pub fn new(window: Arc<ApplicationWindow>) -> Self {
        let stack = Arc::new(Stack::new());
        let stack_switcher = Arc::new(StackSwitcher::new());

        Self::new_webview(Arc::clone(&stack), Arc::clone(&stack_switcher), Arc::clone(&window));

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
            Self::new_webview(stack_clone, stack_switcher_clone, Arc::clone(&window));
        });

        let close_btn = Arc::new(gtk::Button::with_label("Close Tab"));
        Self::handle_close_tab(Arc::clone(&close_btn), Arc::clone(&stack));
        vbox.add(&*close_btn);

        Browser {
            headerbar,
            browser: vbox,
            stack,
        }
    }

    fn new_webview(stack: Arc<Stack>, stack_switcher: Arc<StackSwitcher>, window: Arc<ApplicationWindow>) {
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

        let new_name_string = Uuid::new_v4().to_string();
        let new_name = new_name_string.as_str();

        stack.add_titled(&webview, new_name, "New Tab");

        // Ensure that GTK processes pending events
        while gtk::events_pending() {
            gtk::main_iteration();
        }

        stack.show_all();
        stack_switcher.set_stack(Some(&*stack));
        stack_switcher.show_all();

        Self::switch_to_child_at_position(
            Arc::clone(&stack),
            stack.children().len().saturating_sub(1),
        );

        Self::connect_with_window_title(Arc::clone(&stack), Arc::clone(&window));
    }

    fn switch_to_child_at_position(stack: Arc<Stack>, position: usize) {
        let children = stack.children();
        if position < children.len() {
            stack.set_visible_child(&children[position]);
        } else {
            println!("Position {} is out of bounds", position);
        }
    }

    fn connect_with_window_title(stack: Arc<Stack>, window: Arc<ApplicationWindow>) {
    let current_widget = 
        // Get the currently visible child from the stack
        stack.visible_child().and_then(|child| {
            // Downcast the child to WebView
            child.dynamic_cast::<WebView>().ok()
        });

    if let Some(webview) = current_widget {
        let window = Arc::clone(&window);
        webview.connect_load_changed(move |webview, _| {
            if let Some(title) = webview.title() {
                window.set_title(&format!("{} — Pluto Browser", title));
            } else {
                window.set_title("New Tab — Pluto Browser");
            }
        });
        }
    }

    fn handle_close_tab(btn: Arc<gtk::Button>, stack: Arc<Stack>) {
        btn.connect_clicked(move |_| {
            let stack = Arc::clone(&stack);
            let current_child = stack.visible_child();
            if let Some(current_child) = current_child {
                if stack.children().len() > 1 {
                    stack.remove(&current_child);
                    Self::switch_to_child_at_position(
                        Arc::clone(&stack),
                        stack.children().len().saturating_sub(1),
                    );
                } else {
                    gtk::main_quit();
                }
            }
        });
    }
}
