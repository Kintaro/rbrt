extern crate rand;

use camera::Camera;
use geometry::{ RayDifferential, Vector };
use intersection::Intersection;
use reflection::{ BsdfSample, Bsdf, BxDFType, Transmission, Reflection, Specular };
use renderer::Renderer;
use sampler::Sample;
use scene::Scene;
use spectrum::Spectrum;

use rand::TaskRng;

pub trait Integrator {
    fn preprocess(scene: &Scene, camera: &Camera) {}
    fn request_samples() {}
}

pub trait SurfaceIntegrator : Integrator {
    fn Li(&self, scene: &Scene, renderer: &Renderer, ray: &RayDifferential,
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
  let f = bsdf.sample_f(&wo, &mut wi, &BsdfSample::from_random(rng), Reflection | Specular);
}

pub fn specular_transmit<'a>(ray: &RayDifferential, bsdf: &'a Bsdf<'a>,
    rng: &mut TaskRng, intersection: &Intersection, renderer: &Renderer,
    scene: &Scene, sample: &Sample) -> Spectrum {
  fail!("not implemented");

  let wo = -ray.ray.d;
  let mut wi = Vector::new(0.0, 0.0, 0.0);
  let p = bsdf.dg_shading.p;
  let n = bsdf.dg_shading.nn;
  let f = bsdf.sample_f(&wo, &mut wi, &BsdfSample::from_random(rng), Transmission | Specular);
}
