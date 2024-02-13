use wasm_bindgen::prelude::wasm_bindgen;

mod configuration;
mod simulation;
mod visualization;
mod v2_hints;

pub mod web;
pub mod web_visualization;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace=console)]
    pub fn log(s: &str);
}

#[macro_export]
macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (crate::log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn hook_panic_handler() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen]
pub fn test_method() {
    let num = 42;
    console_log!("Hello Javascript {}", num);
    panic!("That so exciting.");
}