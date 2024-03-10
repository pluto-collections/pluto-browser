use gtk::prelude::ContainerExt;

use crate::{components::searchbar::SearchType, utils::get_search_type};

#[test]
fn get_search_type_test() {
    let strings_with_types = [
        ("https://www.google.com", SearchType::Url),
        ("http://localhost:8000", SearchType::Url),
        ("google", SearchType::Search),
        ("http://google.com", SearchType::Url),
        ("https", SearchType::Search),
        ("ftp://google.com", SearchType::Search),
        ("http://google.com", SearchType::Url),
    ];

    for (string, search_type) in strings_with_types.iter() {
        assert_eq!(search_type, &get_search_type(&string.to_string()));
    }
}
