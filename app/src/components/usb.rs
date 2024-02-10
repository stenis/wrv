use leptos::*;

#[component]
pub fn ConnectUSB(text: &'static str) -> impl IntoView {
    let init = move |_| { 
        
        cfg_if::cfg_if! {
            if #[cfg(not(feature = "ssr"))] {
                spawn_local(async move {
                    let res = airspy::open_async().await;
                    match res {
                        Ok(spy) => {
                            logging::log!("{:?}", spy.device.product_name());
                            let sample_rates = spy.read_samplerates().await.unwrap();
                            for rate in &sample_rates {
                                logging::log!(">> {}", rate);
                            }
                            let _ = spy.start().await.unwrap();
                            let r = spy.set_freq(103_000_000).await.unwrap();
                            logging::log!("{:?}", r.status());
                            let mut i = 0;
                            loop {
                                i+=1;
                                if i > 100 { break; }
                                let buffer = spy.read_bulk(64_000).await.unwrap();
                                logging::log!("{}", buffer[0]);
                            } 
                        },
                        Err(e) => logging::log!("{:?}", e)
                    }
                });
            }
        }
    };

    view! {
        <button type="button"
                on:click=init
                class="w-fit mx-auto my-3 bg-cyan-200 hover:bg-cyan-100 text-black py-1 px-2 rounded">
            {text}
        </button>
    }
}
