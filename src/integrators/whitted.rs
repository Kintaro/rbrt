extern crate rand;
extern crate rbrtcore;

use rbrtcore::camera::Camera;
use rbrtcore::geometry::RayDifferential;
use rbrtcore::integrator::Integrator;
use rbrtcore::integrator::SurfaceIntegrator;
use rbrtcore::intersection::Intersection;
use rbrtcore::renderer::Renderer;
use rbrtcore::sampler::Sample;
use rbrtcore::scene::Scene;
use rbrtcore::spectrum::Spectrum;
use rbrtcore::spectrum::RgbSpectrum;

use rand::Rng;

pub struct WhittedIntegrator {
  max_depth: uint
}

impl Integrator for WhittedIntegrator {

}

impl WhittedIntegrator {
  pub fn new(max_depth: uint) -> WhittedIntegrator {
    WhittedIntegrator { max_depth: max_depth }
  }
}

impl SurfaceIntegrator for WhittedIntegrator {
  fn Li(&self, scene: &Scene, renderer: &Renderer, ray: &RayDifferential,
      intersection: &mut Intersection, sample: &Sample, rng: Box<Rng>) -> Spectrum {
    // Evaluate bsdf at hit point
    let bsdf = intersection.get_bsdf(ray);

    // Initialize common variables for whitted integrator
    let p = bsdf.dg_shading.p;
    let n = bsdf.dg_shading.nn;
    let wo = -ray.ray.d;

    // Compute emitted light if ray hit an area light source
    let mut L = RgbSpectrum::new(0.0);

    return box L;
  }
}
