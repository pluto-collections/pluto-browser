use super::{button, headerbar::load_about_pages};
use glib::{clone, Cast};
use gtk::{
    prelude::{BoxExt, ButtonExt, StackExt, StackSwitcherExt},
    StackSwitcher,
};
use std::{cell::Cell, sync::Arc};
use webkit2gtk::{LoadEvent, WebView, WebViewExt};

#[derive(Clone)]
pub struct ActionButtons {
    container: Arc<gtk::Box>,
    previous_button: Arc<button::WebViewButton>,
    next_button: Arc<button::WebViewButton>,
    refresh_button: Arc<button::WebViewButton>,
    home_button: Arc<button::WebViewButton>,
    is_refreshing: Arc<Cell<bool>>,
}

impl ActionButtons {
    pub fn new() -> Self {
        let previous_button = Arc::new(button::WebViewButton::new(Some("go-previous")));
        let next_button = Arc::new(button::WebViewButton::new(Some("go-next")));
        let refresh_button = Arc::new(button::WebViewButton::new(Some("view-refresh")));
        let home_button = Arc::new(button::WebViewButton::new(Some("go-home")));

        let hbox = Arc::new(gtk::Box::new(gtk::Orientation::Horizontal, 0));
        hbox.pack_start(&previous_button.button, false, false, 4);
        hbox.pack_start(&next_button.button, false, false, 4);
        hbox.pack_start(&refresh_button.button, false, false, 4);
        hbox.pack_start(&home_button.button, false, false, 4);

        ActionButtons {
            is_refreshing: Arc::new(Cell::new(false)),
            previous_button,
            next_button,
            refresh_button,
            home_button,
            container: hbox,
        }
    }

    pub fn get_widget(&self) -> Arc<gtk::Box> {
        Arc::clone(&self.container)
    }

    pub fn connect_action_with_browser(&self, stack_switcher: Arc<StackSwitcher>) {
        // Assume `stack` is a gtk::Stack instance stored in the struct
        let current_widget = stack_switcher.stack().and_then(|stack| {
            // Get the currently visible child from the stack
            stack.visible_child().and_then(|child| {
                // Downcast the child to WebView
                child.dynamic_cast::<WebView>().ok()
            })
        });

        if current_widget.is_none() {
            return;
        }

        let webview = current_widget.unwrap();

        self.previous_button
            .button
            .connect_clicked(clone!(@strong webview => move |_| {
                webview.go_back();
            }));
        // Connect the button to the webview
        self.next_button
            .button
            .connect_clicked(clone!(@strong webview => move |_| {
                webview.go_forward();
            }));

        let is_refreshing = Arc::clone(&self.is_refreshing);
        self.refresh_button
            .button
            .connect_clicked(clone!(@strong webview => move |_| {
                if is_refreshing.get() {
                    webview.stop_loading();
                } else {
                    webview.reload();
                }
            }));

        self.home_button
            .button
            .connect_clicked(clone!(@strong webview => move |_| {
                load_about_pages(&"about:home".to_string(), Arc::clone(&stack_switcher));
            }));

        let refresh_button = Arc::clone(&self.refresh_button);
        let is_refreshing = Arc::clone(&self.is_refreshing);
        webview.connect_load_changed(move |_, load_event: LoadEvent| match load_event {
            LoadEvent::Started => {
                refresh_button.set_icon_name("process-stop");
                is_refreshing.set(true);
            }
            LoadEvent::Finished => {
                refresh_button.set_icon_name("view-refresh");
                is_refreshing.set(false);
            }
            _ => {}
        });
    }
}
