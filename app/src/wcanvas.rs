#![allow(dead_code)]

use leptos::*;
use leptos_use::{use_mouse, UseMouseReturn};

#[component]
pub fn WCanvas() -> impl IntoView {

    let UseMouseReturn {
        x, y, ..
    } = use_mouse();
    
    let parent_ref = create_node_ref::<html::Div>();
    //let (value, set_value) = create_signal(false);
    
    let init = move |_| { 
    cfg_if::cfg_if! {
        if #[cfg(not(feature = "ssr"))] {
                spawn_local(async move {
                    let res = airspy::open_async().await;
                    match res {
                        Ok(airspy) => {
                            logging::log!("{:?}", airspy.device.product_name());
                            let sample_rates = airspy.read_samplerates(|s| { logging::log!("{}", s) }).await.unwrap();
                            for rate in &sample_rates {
                                logging::log!("{}", rate);
                            }       
                        },
                        Err(e) => logging::log!("{:?}", e)
                    }
                });
            }
        }
    };

    cfg_if::cfg_if! {
        if #[cfg(not(feature = "ssr"))] {
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
        <button type="button"
                on:click=init
                class="w-fit mx-auto my-3 bg-cyan-200 hover:bg-cyan-100 text-black py-1 px-2 rounded">connect usb</button>
        <p class="w-96 mx-auto bg-cyan-200">
            {move || {format!(r#"    x: {} y: {}"#, x(), y(),)}}
        </p>
        <div _ref=parent_ref class="w-96 h-96 mx-auto bg-gray-500/30" />
    }
}