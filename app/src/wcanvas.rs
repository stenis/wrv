#![allow(dead_code)]

use leptos::*;

#[component]
pub fn WCanvas() -> impl IntoView {
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
        <div _ref=parent_ref class="w-96 h-96 mx-auto bg-gray-500/30" />
    }
}