use gtk::prelude::ButtonExt;

#[derive(Debug)]
pub struct WebViewButton {
    pub button: gtk::Button,
}

impl WebViewButton {
    pub fn new(label: Option<&str>) -> Self {
        Self {
            button: gtk::Button::from_icon_name(label, gtk::IconSize::Button),
        }
    }

    pub fn set_icon_name(&self, icon_name: &str) {
        self.button.set_image(Some(&gtk::Image::from_icon_name(
            Some(icon_name),
            gtk::IconSize::Button,
        )));
    }
}
