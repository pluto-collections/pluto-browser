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

pub fn get_search_type(entry: &String) -> SearchType {
    // add https:// to the entry if it doesn't have it
    let entry = if entry.starts_with("http") {
        entry.clone()
    } else {
        format!("https://{}", entry)
    }
    .to_lowercase();

    let re = regex::Regex::new(
        r"https?:\/\/(www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()!@:%_\+.~#?&\/\/=]*)",
    ).unwrap_or_else(|_| {
        panic!("Invalid regex");
    });

    // regex for localhosts
    let re_localhost = regex::Regex::new(r"https?:\/\/localhost(:\d+)?").unwrap_or_else(|_| {
        panic!("Invalid regex");
    });

    if re.is_match(&entry) || re_localhost.is_match(&entry) {
        return SearchType::Url;
    }

    SearchType::Search
}

pub fn add_http_to_entry(entry: &String) -> String {
    if entry.starts_with("http") {
        entry.clone()
    } else {
        format!("https://{}", entry)
    }
}

pub fn get_url(entry: &String) -> String {
    let search_type = get_search_type(&entry);
    match search_type {
        SearchType::Url => add_http_to_entry(entry),
        // SearchType::Search => format!("https://www.google.com/search?q={}", entry),
        SearchType::Search => {
            url::Url::parse_with_params("https://www.google.com/search", &[("q", entry)])
                .unwrap()
                .to_string()
        }
    }
}
