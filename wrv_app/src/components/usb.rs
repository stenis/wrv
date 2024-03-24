#![allow(dead_code)]
use std::sync::{Arc, Mutex};

use leptos::*;
use web_sys::MouseEvent;

const U16_SAMPLES_COUNT : u32 = 65_536; // u16
const FFT_SIZE : u32 = 1024;
const FFT_COUNT : u32 = U16_SAMPLES_COUNT / 2 / 2 / FFT_SIZE;
const SAMPLE_RESOLUTION : u32 = 12;
const SAMPLE_ENCAPSULATION : u32 = 16;
const SAMPLE_SHIFT : u32  = SAMPLE_ENCAPSULATION - SAMPLE_RESOLUTION;
const SAMPLE_SCALE : f32 = 1.0 / (1 << (15 - SAMPLE_SHIFT)) as f32;

#[cfg(not(feature = "ssr"))]
fn init(_ : MouseEvent) {
    spawn_local(async move {
        let res = wrv_airspy::open_async().await;
        match res {
            Ok(spy) => {
                logging::log!(">> {}", FFT_COUNT);

                let mut dsp = wrv_dsp::Dsp::new(FFT_SIZE);
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
                    
                    let buffer = spy.read_bulk(U16_SAMPLES_COUNT).await.unwrap();
                    let mut buffer_f = buffer.iter().map(|&v| {
                        v.wrapping_sub(2048) as f32 * SAMPLE_SCALE }).collect::<Vec<f32>>();

                    for i in 0..FFT_COUNT {
                        let s = (i * 2 * FFT_SIZE) as usize;
                        let e = s + (2 * FFT_SIZE as usize);
                        dsp.process(&mut buffer_f[s..e]);
                    }

                    logging::log!("{}", buffer_f[0]);
                }
                logging::log!("reading done.");
            },
            Err(e) => logging::log!("{:?}", e)
        }
    });
}

#[cfg(feature = "ssr")]
fn init(_ : MouseEvent) {}

#[component]
pub fn ConnectUSB(image: Arc<Mutex<[u8; 1024*256]>>, text: &'static str) -> impl IntoView {
    view! {
        <button type="button"
                on:click=init
                class="w-fit mx-auto my-3 bg-cyan-200 hover:bg-cyan-100 text-black py-1 px-2 rounded">
            {text}
        </button>
    }
}
