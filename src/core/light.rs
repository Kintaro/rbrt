use geometry::{ Point, Normal, Vector, RayDifferential, round_up_pow_2 };
use scene::Scene;
use spectrum::Spectrum;
use transform::Transform;

pub struct LightBase {
  num_samples: uint,
  light_to_world: Transform,
  world_to_light: Transform
}

pub trait Light<'a> {
  fn get_base(&'a self) -> LightBase;
  fn get_base_mut(&'a mut self) -> LightBase;

  fn is_delta_light(&'a self) -> bool;

  fn power(&'a self, scene: &Scene) -> Spectrum;
  fn Le(&'a self, ray: &RayDifferential) -> Spectrum;
  fn pdf(&'a self, p: &Point, wi: &Vector) -> f32;

  fn sh_project(&'a self, p: &Point, p_epsilon: f32, lmax: uint, scene: &Scene,
      compute_light_visibility: bool, time: f32, coeffs: &mut Vec<Spectrum>) {
    let ns = round_up_pow_2(self.get_base().num_samples);

    for i in range(0, ns) {

    }
  }
}

pub trait AreaLight<'a> : Light<'a> {
  fn L(&self, p: &Point, n: &Normal, w: &Vector) -> Spectrum;
}
