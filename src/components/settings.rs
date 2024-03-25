// Need to implement this as a function for browser and tabs. Haven't done it yet.
// Also not included in the mod.rs file.

use webkit2gtk::{Settings, SettingsExt, WebView, WebViewExt};
#[derive(Clone)]
pub struct WebViewSettings {
    settings: Settings,
}

impl WebViewSettings {
    pub fn new() -> Self {
        let webview = WebView::new();
        let settings = WebViewExt::settings(&webview).unwrap();
        WebViewSettings { settings }
    }

    pub fn get_settings(&self) {
        self.settings.set_enable_developer_extras(true);
        self.settings
            .set_javascript_can_open_windows_automatically(true);
        self.settings.set_allow_modal_dialogs(true);
        self.settings.set_auto_load_images(true);
        self.settings.set_enable_caret_browsing(false);
        self.settings.set_enable_developer_extras(true);
        self.settings.set_enable_dns_prefetching(true);
        self.settings.set_enable_fullscreen(true);
        self.settings.set_enable_html5_database(true);
        self.settings.set_enable_html5_local_storage(true);
        self.settings.set_enable_hyperlink_auditing(true);
        self.settings.set_enable_javascript(true);
        self.settings.set_enable_offline_web_application_cache(true);
        self.settings.set_enable_page_cache(true);
        self.settings.set_enable_resizable_text_areas(true);
        self.settings.set_enable_site_specific_quirks(true);
        self.settings.set_enable_smooth_scrolling(true);
        self.settings.set_enable_tabs_to_links(true);
        self.settings.set_enable_webaudio(true);
        self.settings.set_enable_webgl(true);
        self.settings.set_javascript_can_access_clipboard(true);
        self.settings
            .set_load_icons_ignoring_image_load_setting(true);
        self.settings.set_media_playback_allows_inline(true);
        self.settings
            .set_media_playback_requires_user_gesture(false);
        self.settings.set_print_backgrounds(true);
        self.settings.set_zoom_text_only(false);
    }
}
