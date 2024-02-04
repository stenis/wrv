#![allow(dead_code)]
use leptos::*;
use leptos_use::{use_mouse, UseMouseReturn};

#[component]
pub fn WCanvas() -> impl IntoView {
    #[allow(unused_variables)]
    let canvas_ref = create_node_ref::<html::Canvas>();
    let UseMouseReturn { x, y, .. } = use_mouse();

    cfg_if::cfg_if! {
        if #[cfg(not(feature = "ssr"))] {
            let _ = create_local_resource(
                || (),
                move |_| async move {
                    gfx::wcanvas::run(&canvas_ref).await;
                    logging::log!("wgpu initialized.");
                },
            );
        }
    }
    
    view! {
        <p class="w-96 mx-auto bg-cyan-200 rounded-t">
            {move || {format!(r#"    x: {} y: {}"#, x(), y(),)}}
        </p>
        <canvas ref=canvas_ref class="w-96 h-96 mx-auto bg-gray-500/30 rounded-b" />
    }
}