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
        ("/home/user/file.txt", SearchType::File),
        ("file:///home/user/file.txt", SearchType::File),
    ];

    for (string, search_type) in strings_with_types.iter() {
        assert_eq!(search_type, &get_search_type(&string.to_string()));
    }
}

#[test]
fn add_http_to_entry_test() {
    let strings_with_http = [
        ("https://www.google.com", "https://www.google.com"),
        ("http://localhost:8000", "http://localhost:8000"),
        ("google", "http://google"),
        ("http://google.com", "http://google.com"),
        ("https", "https"),
        ("ftp://google.com", "http://ftp://google.com"),
        ("http://google.com", "http://google.com"),
    ];

    for (string, http_string) in strings_with_http.iter() {
        assert_eq!(
            http_string,
            &crate::utils::add_http_to_entry(&string.to_string())
        );
    }
}

#[test]
fn is_child_in_box_test() {
    let _ = gtk::init();
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    let button = gtk::Button::new();
    vbox.add(&button);

    assert_eq!(true, crate::utils::is_child_in_box(&vbox, &button));
}
