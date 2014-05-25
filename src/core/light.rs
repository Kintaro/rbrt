use geometry::{ Point, Normal, Vector, Ray, RayDifferential, round_up_pow_2 };
use renderer::Renderer;
use sampler::Sample;
use scene::Scene;
use spectrum::Spectrum;
use transform::Transform;

use rand::{ TaskRng, Rng };

pub struct LightSample {
  upos: (f32, f32),
  ucomponent: f32
}

impl LightSample {
  pub fn from_random(rng: &mut TaskRng) -> LightSample {
    LightSample { upos: rng.gen(), ucomponent: rng.gen() }
  }
}

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

  fn sample_l(&'a self, p: &Point, p_epsilon: f32, ls: &LightSample, time: f32,
      wi: &mut Vector, pdf: &mut f32, vis: &mut VisibilityTester) -> Spectrum;
}

pub trait AreaLight<'a> : Light<'a> {
  fn L(&self, p: &Point, n: &Normal, w: &Vector) -> Spectrum;
}

pub struct VisibilityTester {
  pub r: Ray
}

impl VisibilityTester {
  pub fn new() -> VisibilityTester {
    VisibilityTester { r: Ray::zero() }
  }

  pub fn unoccluded(&self, scene: &Scene) -> bool {
    fail!("not implemented");
  }

  pub fn transmittance(&self, scene: &Scene, renderer: &Renderer, sample: &Sample,
      rng: &mut TaskRng) -> Spectrum {
    fail!("not implemented");
  }
}
