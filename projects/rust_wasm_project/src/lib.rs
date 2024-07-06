use wasm_bindgen::prelude::*;
use std::panic;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // Set the panic hook to get better error messages in the console
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    Ok(())
}
