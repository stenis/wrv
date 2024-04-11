#![allow(dead_code)]
use leptos::*;
use web_sys::MouseEvent;

const BYTE_COUNT : u32 = 65_536; // u8
const F32_SAMPLES_COUNT : usize = 65_536 / 2;
const FFT_SIZE : u32 = 1024;
const FFT_COUNT : u32 = BYTE_COUNT / 2 / 2 / FFT_SIZE;
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
                let mut buffer_f = [0.0; F32_SAMPLES_COUNT];
                let mut min = f32::MAX;
                let mut max = f32::MIN;

                while i < 100 {
                    i += 1;
                    
                    let buffer = spy.read_bulk(BYTE_COUNT).await.unwrap();
                    
                    //convert to f32
                    for i in 0..buffer_f.len() {
                        buffer_f[i] = buffer[i].wrapping_sub(2048) as f32 * SAMPLE_SCALE;
                    }
                    
                    // real -> complex
                    let buffer_c = dsp.real_to_complex(&mut buffer_f);
                    
                    for i in 0..FFT_COUNT {
                        let s = (i  * FFT_SIZE) as usize;
                        let e = s + (FFT_SIZE as usize);
                        dsp.process(&mut buffer_c[s..e]);
                        for &v in buffer_c[s..e].iter() {
                            let v = v.norm_sqr();
                            if v > max { max = v; }
                            if v < min { min = v; }
                        }
                    }

                    logging::log!("v: {} min: {min} max: {max}", buffer_c[0].norm_sqr());
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
pub fn ConnectUSB(text: &'static str) -> impl IntoView {
    view! {
        <button type="button"
                on:click=init
                class="w-fit mx-auto my-3 bg-cyan-200 hover:bg-cyan-100 text-black py-1 px-2 rounded">
            {text}
        </button>
    }
}
