mod utils;
mod logic;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{Clamped, JsCast};
use web_sys::{ImageData};

use color_space::*;
use color_space::ToRgb;
use color_space::Rgb;

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

#[wasm_bindgen]
pub fn generate_mandelbrot_set(
  canvas_w: usize,
  canvas_h: usize,
  x_min: f64,
  x_max: f64,
  y_min: f64,
  y_max: f64,
  max_iter: usize,
) -> Vec<u8> {
  measure_elapsed_time!("generate:wasm\telapsed:", {
    generate_mandelbrot_set2(canvas_w, canvas_h,
      x_min, x_max, y_min, y_max, max_iter)
  })
}


fn get_n_diverged(x0: f64, y0: f64, max_iter: usize) -> u8 {
  // 複素数z_nの実部をxn,虚部をynとします
  let mut xn = 0.0;
  let mut yn = 0.0;
  for i in 1..max_iter {
    let x_next = xn * xn - yn * yn + x0;
    let y_next = 2.0 * xn * yn + y0;

    xn = x_next;
    yn = y_next;
    if yn * yn + xn * xn > 4.0 {
      return i as u8; // 複素数の絶対値が2を超えると発散と判定
    }
  }
  max_iter as u8
}


pub fn generate_mandelbrot_set2(
  canvas_w: usize,
  canvas_h: usize,
  x_min: f64,
  x_max: f64,
  y_min: f64,
  y_max: f64,
  max_iter: usize,
) -> Vec<u8> {
  // JSの8bit符号なし整数の配列であるUint8ClampledAllay型をつくりたいため、Vec<u8>で色情報を作る
  let mut data = vec![];
  for i in 0..500 {
    for _ in 0..canvas_w {
      // let v = (255 % j) as f64;
      let v = i as f64;

      let hsv = Hsv::new(v as f64,1.0,1.0);
      let rgb = Rgb::from_color(&hsv);

      data.push(rgb.r as u8); // R
      data.push(rgb.g as u8); // G
      data.push(rgb.b as u8); // B
      data.push(255); // A
    }
  }
  data
}


#[wasm_bindgen]
pub fn draw_mandelbrot_set(){
  log("A");
  // const CANVAS_ID :&str = "canvas_wasm";
  const CANVAS_ID :&str = "canvas_hybrid";
  let document = web_sys::window().unwrap().document().unwrap();
  let canvas = document.get_element_by_id(CANVAS_ID).unwrap();
  // HtmlCanvasElement型のAPIを使うためにElement型からキャスト
  let canvas: web_sys::HtmlCanvasElement = canvas
    .dyn_into::<web_sys::HtmlCanvasElement>()
    .map_err(|_| ())
    .unwrap();
  // Object型からCanvasRenderingContext2d型にキャスト
  let context = canvas
    .get_context("2d") // Result<Option<Object>, JsValue>
    .unwrap() // Option<Object>
    .unwrap() // Object
    .dyn_into::<web_sys::CanvasRenderingContext2d>()
    .unwrap();
  let canvas_w = canvas.width() as usize;
  let canvas_h = canvas.height() as usize;
  const X_MIN: f64 = -1.5;
  const X_MAX: f64 = -0.5;
  const Y_MIN: f64 = -1.0;
  const Y_MAX: f64 = -1.0;
  const MAX_ITER: usize = 64;

  let mut result = measure_elapsed_time!("generate:wasm\telapsed:",{
    generate_mandelbrot_set2(canvas_w, canvas_h,
      X_MIN, X_MAX, Y_MIN, Y_MAX, MAX_ITER)
  });
  measure_elapsed_time!("generate:wasm\telapsed:",{
    let data = ImageData::new_with_u8_clamped_array_and_sh(
      Clamped(&mut result),
      canvas.width(),
      canvas.height(),
    );
    if let Ok(data) = data {
      let _ = context.put_image_data(&data, 0.0, 0.0);
    }
  })
}