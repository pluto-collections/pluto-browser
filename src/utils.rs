use gtk::prelude::*;
use gtk::Box;

pub fn is_child_in_box<T: IsA<gtk::Widget>>(parent: &Box, child: &T) -> bool {
    let children = parent.children();
    for widget in children {
        if widget == *child {
            return true;
        }
    }
    false
}
