#![allow(dead_code)]

use leptos::*;

#[component]
pub fn WCanvas() -> impl IntoView {
    cfg_if::cfg_if! {
        if #[cfg(not(feature = "ssr"))] {

            let _ = create_local_resource(
                || (),
                |_| async move {
                    gfx::wcanvas::run().await;
                    logging::log!("wgpu initialized.");
                },
            );
        }
    }
    view! {
        <div on:click=|_| { logging::log!("HEY clicked."); }>
            "CANVAS GOES HERE!"
        </div>
    }
}