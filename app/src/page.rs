use leptos::{ev::click, *};
use wasm_bindgen::JsCast;
use web_sys::{HtmlButtonElement, HtmlDivElement, MouseEvent, ScrollBehavior, ScrollIntoViewOptions};

#[component]
pub fn Page(heading: &'static str, children: Children) -> impl IntoView {
    
    let this_ref = create_node_ref::<html::Div>();
    let next = move |_| {
                    //let x = e.current_target().unwrap();//.dyn_ref::<HtmlButtonElement>().unwrap().clone();
                    let x : &HtmlDivElement = &*this_ref.get_untracked().unwrap();
                    if let Some(n) = x.next_element_sibling() {
                        logging::log!("{:#?}", &n);
                        let mut ops = ScrollIntoViewOptions::new();
                        ops.behavior(ScrollBehavior::Smooth);
                        n.scroll_into_view_with_scroll_into_view_options(&ops);
                    }
                };
    view! {
        <div class="h-screen" _ref=this_ref>
            <h1 class="my-2 font-thin text-3xl text-sky-200/90">{heading}</h1>
            {children()}
            <button on:click=next>
                <svg class="m-1 stroke-sky-200" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M19.5 13.5 12 21m0 0-7.5-7.5M12 21V3" />
                </svg>
            </button>
        </div>
    }
}