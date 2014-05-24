pub struct FilterBase {
  pub x_width: f32,
  pub y_width: f32,
  pub inv_x_width: f32,
  pub inv_y_width: f32,
}

impl FilterBase {
  pub fn new(x: f32, y: f32) -> FilterBase {
    FilterBase {
      x_width: x,
      y_width: y,
      inv_x_width: 1.0 / x,
      inv_y_width: 1.0 / y
    }
  }
}

pub trait Filter {
  fn evaluate(x: f32, y: f32) -> f32;
}
