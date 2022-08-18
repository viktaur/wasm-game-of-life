mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_all&oc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: String);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(String::from("Hello, ") + name);
}
