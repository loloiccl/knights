extern crate js_sys;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn apply(f: &js_sys::Function) {
    let _ = f.call1(&JsValue::NULL, &JsValue::from(42));
}
