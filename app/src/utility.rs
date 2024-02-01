use leptos::{ev::click, *};
use web_sys::{ScrollBehavior, ScrollIntoViewOptions};

pub fn options() -> ScrollIntoViewOptions {
    let mut ops = ScrollIntoViewOptions::new();
    ops.behavior(ScrollBehavior::Smooth);
    ops
}

// pub fn scroll_to_next_element() {
//     if let Some(next_elem) = (&*this_ref.get_untracked().unwrap()).next_element_sibling() {
//         next_elem.scroll_into_view_with_scroll_into_view_options(&options());
//     }
// }