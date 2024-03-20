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
}
