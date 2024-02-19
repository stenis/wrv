#![allow(dead_code)]
use std::sync::{Arc, Mutex};

use leptos::*;
use leptos_use::{use_mouse_in_element, UseMouseInElementReturn };

#[component]
pub fn WCanvas() -> impl IntoView {
    let color = Arc::new(Mutex::new(0.0));
    let effect_color = color.clone();
    let canvas_ref = create_node_ref::<html::Canvas>();
    
    let UseMouseInElementReturn  { element_x, element_y, element_width, element_height, .. } = use_mouse_in_element(canvas_ref);
    let x = move || element_x().clamp(0.0, element_width()).floor();
    let y = move || element_y().clamp(0.0, element_height()).floor();

    create_effect(move |_| {
        let mut c = effect_color.lock().unwrap();
        *c = element_x() / element_width();
    });

    cfg_if::cfg_if! {
        if #[cfg(not(feature = "ssr"))] {
            spawn_local(async move { 
                let color = color.clone();
                wrv_gfx::run(&canvas_ref, color).await;
                logging::log!("wgpu initialized.");
            });
        }
    }
    
    view! {
        <p class="w-96 mx-auto bg-cyan-200 rounded-t">
            {move || {format!(r#"    x: {} y: {}"#, x(), y(),)}}
        </p>
        <canvas ref=canvas_ref class="w-96 h-96 mx-auto bg-gray-500/30 rounded-b" />
    }
}