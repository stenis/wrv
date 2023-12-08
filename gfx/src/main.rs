use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
fn main() {
    pollster::block_on(gfx::run());
}