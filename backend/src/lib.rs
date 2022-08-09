use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn double(num: u8) -> u8 {
    num * 2u8
}
