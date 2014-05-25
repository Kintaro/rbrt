extern crate rand;

use camera::Camera;
use geometry::{ RayDifferential, Vector, abs_dot };
use intersection::Intersection;
use reflection::{ BsdfSample, Bsdf, BxDFType, Transmission, Reflection, Specular };
use renderer::Renderer;
use sampler::Sample;
use scene::Scene;
use spectrum::{ RgbSpectrum, Spectrum };

use rand::TaskRng;

pub trait Integrator {
  fn preprocess(scene: &Scene, camera: &Camera) {}
  fn request_samples() {}
}

pub trait SurfaceIntegrator : Integrator {
  fn Li<'a>(&self, scene: &'a Scene<'a>, renderer: &Renderer, ray: &RayDifferential,
    intersection: &mut Intersection, sample: &Sample, rng: &mut TaskRng) -> Spectrum;
}

pub fn specular_reflect<'a>(ray: &RayDifferential, bsdf: &'a Bsdf<'a>,
  rng: &mut TaskRng, intersection: &Intersection, renderer: &Renderer,
  scene: &Scene, sample: &Sample) -> Spectrum {
  fail!("not implemented");

  let wo = -ray.ray.d;
  let mut wi = Vector::new(0.0, 0.0, 0.0);
  let p = bsdf.dg_shading.p;
  let n = bsdf.dg_shading.nn;
  let mut pdf = 0.0;
  let (f, _) = bsdf.sample_f(&wo, &mut wi, &BsdfSample::from_random(rng),
    &mut pdf, &mut(Reflection | Specular));

  let mut l = RgbSpectrum::new(0.0);

  if pdf > 0.0 && !f.is_black() && abs_dot(wi, n) != 0.0 {

  }

  return box l;
}

pub fn specular_transmit<'a>(ray: &RayDifferential, bsdf: &'a Bsdf<'a>,
  rng: &mut TaskRng, intersection: &Intersection, renderer: &Renderer,
  scene: &Scene, sample: &Sample) -> Spectrum {
  fail!("not implemented");

  let wo = -ray.ray.d;
  let mut wi = Vector::new(0.0, 0.0, 0.0);
  let p = bsdf.dg_shading.p;
  let n = bsdf.dg_shading.nn;
  let mut pdf = 0.0;
  let (f, _) = bsdf.sample_f(&wo, &mut wi, &BsdfSample::from_random(rng),
    &mut pdf, &mut (Transmission | Specular));
}
