#![allow(dead_code)]
use std::sync::{Arc, Mutex};

use leptos::*;
use leptos_use::{use_mouse_in_element, UseMouseInElementReturn };

#[component]
pub fn WCanvas() -> impl IntoView {
    #[allow(unused_variables)]
    let canvas_ref = create_node_ref::<html::Canvas>();

    let UseMouseInElementReturn  { x, y, element_width, .. } = use_mouse_in_element(canvas_ref);
    let color = Arc::new(Mutex::new(0.0));
    let c = color.clone();    
    
    create_effect(move |_| {
        let mut c = c.lock().unwrap();
        *c += x.get() / element_width();
        if *c > 1.0 { *c = 0.0; }
    });

    cfg_if::cfg_if! {
        if #[cfg(not(feature = "ssr"))] {
            let _ = create_local_resource(
                || (),
                move |_| {
                    let color = color.clone();
                    async move {
                        gfx::wcanvas::run(&canvas_ref, color).await;
                        logging::log!("wgpu initialized.");
                    }
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