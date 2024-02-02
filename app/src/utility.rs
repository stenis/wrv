use leptos::{ev::click, *};
use web_sys::{KeyboardEvent, ScrollBehavior, ScrollToOptions};

pub fn keyboard_scroll(ev: KeyboardEvent) {
    let _ : Option<_> = try {
        let h : f64 = window().inner_height().ok()?.as_f64()?;
        let mut ops = ScrollToOptions::new();
        ops.behavior(ScrollBehavior::Smooth);

        match ev.key_code() {
            37 => window().scroll_by_with_scroll_to_options(&ops.top(-h)), // left -> previous
            39 => window().scroll_by_with_scroll_to_options(&ops.top(h)), // right -> next
            _ => (),
        }
    };
}