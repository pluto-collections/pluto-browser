use gtk::prelude::HeaderBarExt;

pub struct Headerbar {
    pub headerbar: gtk::HeaderBar,
}

impl Headerbar {
    pub fn new() -> Self {
        let headerbar = gtk::HeaderBar::new();
        headerbar.set_title(Some("Pluto Browser"));
        headerbar.set_show_close_button(true);

        Headerbar { headerbar }
    }

    pub fn get_widget(&self) -> &gtk::HeaderBar {
        &self.headerbar
    }

    pub fn set_title(&self, title: String) {
        self.headerbar.set_title(Some(title.as_str()));
    }
}
