use gtk::prelude::*;
use gtk::Box;

use crate::components::searchbar::SearchType;

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

#[allow(dead_code)]
pub fn add_https_to_entry(entry: &String) -> String {
    // is localhost
    if entry.contains("localhost") {
        return add_http_to_entry(entry);
    }

    if entry.starts_with("http") {
        entry.clone()
    } else {
        format!("https://{}", entry)
    }
}
pub fn add_http_to_entry(entry: &String) -> String {
    if entry.starts_with("http") {
        entry.clone()
    } else {
        format!("http://{}", entry)
    }
}

#[allow(dead_code)]
pub fn is_child_in_box<T: IsA<gtk::Widget>>(parent: &Box, child: &T) -> bool {
    let children = parent.children();
    for widget in children {
        if widget == *child {
            return true;
        }
    }
    false
}
