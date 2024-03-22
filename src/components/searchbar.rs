use std::rc::Rc;

use gtk::prelude::{EntryExt, WidgetExt};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum SearchType {
    Url,
    Search,
    File,
}

#[derive(Clone)]
pub struct SearchBar {
    widget: gtk::Entry,
}

impl SearchBar {
    pub fn new(_css_provider: Rc<gtk::CssProvider>) -> Self {
        // Create a new SearchEntry
        let search_entry = gtk::Entry::new();
        search_entry.set_hexpand(true);
        search_entry.set_placeholder_text(Some("Search or enter address"));

        SearchBar {
            widget: search_entry,
        }
    }

    pub fn get_widget(&self) -> &gtk::Entry {
        &self.widget
    }
}

pub fn get_url(entry: &String) -> String {
    let search_type = crate::utils::get_search_type(&entry);
    match search_type {
        SearchType::Url => crate::utils::add_http_to_entry(entry),
        SearchType::Search => {
            url::Url::parse_with_params("https://www.google.com/search", &[("q", entry)])
                .unwrap()
                .to_string()
        }
        SearchType::File => {
            if entry.starts_with("file://") {
                entry.trim().to_string()
            } else {
                format!("file://{}", entry.trim())
            }
        }
    }
}
