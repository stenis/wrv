#![allow(dead_code)]

use leptos::*;
use leptos_use::{use_mouse, UseMouseReturn};

#[component]
pub fn WCanvas() -> impl IntoView {

    let UseMouseReturn {
        x, y, ..
    } = use_mouse();

    cfg_if::cfg_if! {
        if #[cfg(not(feature = "ssr"))] {
            
            let parent_ref = create_node_ref::<html::Div>();

            let _ = create_local_resource(
                || (),
                move |_| async move {
                    gfx::wcanvas::run(&parent_ref).await;
                    logging::log!("wgpu initialized.");
                },
            );
        }
    }
    view! {
        <p class="w-96 mx-auto bg-cyan-200">
            {move || {format!(r#"    x: {} y: {}"#, x.get(), y.get(),)}}
        </p>
        <div _ref=parent_ref class="w-96 h-96 mx-auto bg-gray-500/30" />
        <div>hello!</div>
        <p>wowz</p>
    }
}