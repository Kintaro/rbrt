use sampler::CameraSampleBase;
use spectrum::Spectrum;

pub struct FilmBase {
  x_resolution: uint,
  y_resolution: uint
}

pub trait Film {
  fn add_sample(sample: &CameraSampleBase, L: &Spectrum);
  fn splat(sample: &CameraSampleBase, L: &Spectrum);
  fn get_sample_extent() -> (uint, uint, uint, uint);
  fn get_pixel_extent() -> (uint, uint, uint, uint);
  fn update_display(x0: uint, y0: uint, x1: uint, y1: uint, splat_scale: Option<f32>);
  fn write_image(splat_scale: Option<f32>);
}
