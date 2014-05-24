extern crate rand;
extern crate rbrtcore;

use rbrtcore::camera::Camera;
use rbrtcore::geometry::RayDifferential;
use rbrtcore::integrator::{
  Integrator,
  SurfaceIntegrator,
  specular_reflect,
  specular_transmit
};
use rbrtcore::intersection::Intersection;
use rbrtcore::renderer::Renderer;
use rbrtcore::sampler::Sample;
use rbrtcore::scene::Scene;
use rbrtcore::spectrum::Spectrum;
use rbrtcore::spectrum::RgbSpectrum;

use rand::TaskRng;

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
      intersection: &mut Intersection, sample: &Sample, rng: &mut TaskRng) -> Spectrum {
    // Evaluate bsdf at hit point
    let bsdf = intersection.get_bsdf(ray);

    // Initialize common variables for whitted integrator
    let p = bsdf.dg_shading.p;
    let n = bsdf.dg_shading.nn;
    let wo = -ray.ray.d;

    // Compute emitted light if ray hit an area light source
    let mut L = intersection.Le(&wo);

    for light in scene.lights.iter() {
      // let li = light.sample_l(p, intersection.ray_epsilon, )
    }

    if ray.ray.depth + 1 < self.max_depth {
      // Trace rays for specular reflection and refraction
      *L = *L + *specular_reflect(ray, &bsdf, rng, intersection, renderer, scene, sample);
      *L = *L + *specular_transmit(ray, &bsdf, rng, intersection, renderer, scene, sample);
    }

    return L;
  }
}
