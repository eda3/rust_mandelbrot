use color_space::*;
use color_space::ToRgb;
use color_space::Rgb;


#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_get_n_diverged() {
    let max_iter = 10;
    assert_eq!(get_n_diverged(1.0, 0.0, max_iter), 3); // 0→1→2→5で発散と判定
    assert_eq!(get_n_diverged(0.0, 0.0, max_iter), max_iter as u8);
    assert_eq!(get_n_diverged(0.0, 1.0, max_iter), max_iter as u8)
  }
  #[test]
  fn test_generate_mandelbrot_set() {
    let canvas_w = 2;
    let canvas_h = 2;
    let x_min = -1.0;
    let x_max = 1.0;
    let y_min = -1.0;
    let y_max = 1.0;
    let max_iter = 8;
    assert_eq!(
      generate_mandelbrot_set(canvas_w, canvas_h, x_min, x_max, y_min, y_max, max_iter),
      vec![96, 96, 96, 255, 0, 0, 0, 255, 0, 0, 0, 255, 0, 0, 0, 255]
    )
  }
}