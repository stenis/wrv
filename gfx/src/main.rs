use gfx::run;

#[cfg_attr(target_arch="wasm32", wasm_bindgen(start))]
fn main() {
    run();
}