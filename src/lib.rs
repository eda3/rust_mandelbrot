mod utils;
mod logic;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{Clamped, JsCast};
use web_sys::ImageData;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
  #[wasm_bindgen(js_namespace = console)]
  fn log(a: &str);
}

macro_rules! console_log {
  ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

macro_rules! measure_elapsed_time{
  ($t:tt,$s:block) => {{
      let windows = web_sys::window().expect("sholud have a window the context");
      let performace = window
        .performance()
        .expect("peformance should be available");
      let start = performance.now();
      console_log("{}:{}[ms]", $t, end - start);
      result
    }};
  }