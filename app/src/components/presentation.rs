use leptos::*;
use web_sys::{ScrollBehavior, ScrollIntoViewOptions};

fn options() -> ScrollIntoViewOptions {
    let mut ops = ScrollIntoViewOptions::new();
    ops.behavior(ScrollBehavior::Smooth);
    ops
}

#[component]
pub fn Slide(
    heading: &'static str,
    #[prop(default = false)] hide_next: bool,
    #[prop(default = false)] hide_previous: bool,
    children: Children) -> impl IntoView {
    
    let this_ref = create_node_ref::<html::Div>();

    let next = move |_| {
        if let Some(next_elem) = (&*this_ref.get_untracked().unwrap()).next_element_sibling() {
            next_elem.scroll_into_view_with_scroll_into_view_options(&options());
        }
    };
    let prev = move |_| {
        if let Some(prev_elem) = (&*this_ref.get_untracked().unwrap()).previous_element_sibling() {
            prev_elem.scroll_into_view_with_scroll_into_view_options(&options());
        }
    };

    view! {
        <div class="h-screen" _ref=this_ref>
            <h1 class="my-2 font-thin text-3xl text-sky-200/90">{heading}</h1>
            {children()}
            
            <Show  
                when=move || !hide_next >
                <button on:click=next class="my-2">
                    <svg class="w-6 h-6 m-1 stroke-sky-200" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
                        <path stroke-linecap="round" stroke-linejoin="round" d="M19.5 13.5 12 21m0 0-7.5-7.5M12 21V3" />
                    </svg>
                </button>
            </Show>
            <Show  
                when=move || !hide_previous >
                    <button on:click=prev class="my-2">
                        <svg class="w-6 h-6 m-1 stroke-sky-200" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" d="M4.5 10.5 12 3m0 0 7.5 7.5M12 3v18" />
                        </svg>
                    </button>
            </Show>        
        </div>
    }
}