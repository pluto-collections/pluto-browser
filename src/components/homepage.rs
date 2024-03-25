// not included in the module tree.
// Need to see this nicely
#[derive(Debug, Clone)]
pub struct HomePage {
    pub browser: browser::Browser,
    pub action_buttons: action_buttons::ActionButtons,
    pub headerbar: headerbar::Headerbar,
}
impl HomePage {
    pub fn new(css_provider: Rc<gtk::CssProvider>) -> Self {
        let browser = browser::Browser::new();
        let action_buttons = action_buttons::ActionButtons::new(css_provider.clone());
        let headerbar = headerbar::Headerbar::new(css_provider.clone());
        headerbar.connect_searchbar_with_browser(Rc::clone(&browser));
        action_buttons.connect_action_with_browser(Rc::clone(&browser));
        HomePage {
            browser,
            action_buttons,
            headerbar,
        }
    }
    pub fn get_browser(&self) -> &browser::Browser {
        &self.browser
    }
    pub fn get_action_buttons(&self) -> &action_buttons::ActionButtons {
        &self.action_buttons
    }
    pub fn get_headerbar(&self) -> &headerbar::Headerbar {
        &self.headerbar
    }
}
