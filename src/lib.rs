mod utils;
mod logic;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{Clamped, JsCast};
use web_sys::{ImageData};

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
      let window = web_sys::window().expect("sholud have a window the context");
      let performance = window
        .performance()
        .expect("peformance should be available");
      let start = performance.now();
      let result = {$s};
      let end = performance.now();
      console_log!("{}:{}[ms]", $t, end - start);
      result
  }};
}

pub fn generage_mandbrot_set(
  canvas_w: usize,
  canvas_h: usize,
  x_min: f64,
  x_max: f64,
  y_min: f64,
  y_max: f64,
  max_iter: usize,
) -> Vec<u8> {
  measure_elapsed_time!("generate:wasm\telapsed:", {
    logic::generate_mandelbrot_set(canvas_w, canvas_h,
      x_min, x_max, y_min, y_max, max_iter)
  })
}
