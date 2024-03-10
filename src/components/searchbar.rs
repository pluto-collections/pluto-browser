#[derive(Debug)]
pub enum SearchType {
    Url,
    Search,
}

#[derive(Clone)]
pub struct SearchBar {
    widget: gtk::Entry,
}

impl SearchBar {
    pub fn new() -> Self {
        // Create a new SearchEntry
        let search_entry = gtk::Entry::new();

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
    }
}
