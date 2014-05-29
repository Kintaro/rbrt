use geometry::{ Point, Normal, Vector, Ray, RayDifferential, distance, round_up_pow_2 };
use montecarlo::{ sample02, van_der_corput };
use renderer::Renderer;
use sampler::Sample;
use scene::Scene;
use spectrum::Spectrum;
use spherical::{ sh_terms, sh_evaluate };
use transform::Transform;

use std::f32::INFINITY;
use rand::{ TaskRng, Rng };

pub struct LightSampleOffsets {
  pub num_samples:      uint,
  pub component_offset: uint,
  pub position_offset:  uint
}

pub struct LightSample {
  upos: (f32, f32),
  ucomponent: f32
}

impl LightSample {
  pub fn new(up0: f32, up1: f32, ucomp: f32) -> LightSample {
    LightSample { upos: (up0, up1), ucomponent: ucomp }
  }

  pub fn from_sample(sample: &Sample, offsets: &LightSampleOffsets, n: uint) -> LightSample {
    let up0 = sample.twoD[offsets.position_offset][2 * n];
    let up1 = sample.twoD[offsets.position_offset][2 * n + 1];
    let ucomp = sample.oneD[offsets.component_offset][n];

    LightSample::new(up0, up1, ucomp)
  }

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
  fn pdf(&'a self, p: &Point, wi: &Vector) -> f32;

  fn Le(&'a self, ray: &RayDifferential) -> Spectrum {
    Spectrum::new(0.0)
  }

  fn sh_project(&'a self, p: &Point, p_epsilon: f32, lmax: uint, scene: &Scene,
      compute_light_visibility: bool, time: f32,
      rng: &mut TaskRng, coeffs_v: &mut Vec<Spectrum>) {
    let ns = round_up_pow_2(self.get_base().num_samples);
    let scramble1D = rng.gen::<uint>();
    let scramble2D = rng.gen::<(uint, uint)>();
    let mut ylm = Vec::from_elem(sh_terms(lmax as int), 0.0f32);
    let len = coeffs_v.len();
    let mut coeffs = coeffs_v.mut_slice(0, len);

    for i in range(0, ns) {
      let mut u = (0.0, 0.0);
      sample02(i, scramble2D, &mut u);
      let light_sample = LightSample::new(u.val0(), u.val1(),
        van_der_corput(i, scramble1D));
      let mut vis = VisibilityTester::new();
      let mut wi = Vector::zero();
      let mut pdf = 0.0;
      let li = self.sample_l(p, p_epsilon, &light_sample, time,
        &mut wi, &mut pdf, &mut vis);

      if !li.is_black() && pdf > 0.0 &&
          (!compute_light_visibility || vis.unoccluded(scene)) {
        sh_evaluate(&wi, lmax as int, &mut ylm);
        for j in range(0, sh_terms(lmax as int)) {
          coeffs[j] = coeffs[j] + (li * *ylm.get(j) / (pdf * ns as f32));
        }
      }
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
    !scene.intersect_p(&self.r)
  }

  pub fn transmittance(&self, scene: &Scene, renderer: &Renderer, sample: &Sample,
      rng: &mut TaskRng) -> Spectrum {
    renderer.transmittance(scene, &RayDifferential::new(&self.r), sample, rng)
  }

  pub fn set_segment(&mut self, p1: &Point, eps1: f32, p2: &Point, eps2: f32, time: f32) {
    let dist = distance(p1, p2);
    self.r = Ray::new(p1, &((p2 - *p1) / dist), eps1, dist * (1.0 - eps2), time);
  }

  pub fn set_ray(&mut self, p: &Point, eps: f32, w: &Vector, time: f32) {
    self.r = Ray::new(p, w, eps, INFINITY, time);
  }
}
