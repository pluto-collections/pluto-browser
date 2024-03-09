pub struct SearchBar {
    widget: gtk::SearchEntry,
}

impl SearchBar {
    pub fn new() -> Self {
        // Create a new SearchEntry
        let search_entry = gtk::SearchEntry::new();

        SearchBar {
            widget: search_entry,
        }
    }

    pub fn get_widget(&self) -> &gtk::SearchEntry {
        &self.widget
    }
}
