use rbrtcore::geometry::{ RayDifferential, Vector, abs_dot };
use rbrtcore::integrator::{
  Integrator,
  SurfaceIntegrator,
  specular_reflect,
  specular_transmit
};
use rbrtcore::intersection::Intersection;
use rbrtcore::light::{ LightSample, VisibilityTester };
use rbrtcore::paramset::ParamSet;
use rbrtcore::reflection::AllTypes;
use rbrtcore::renderer::Renderer;
use rbrtcore::sampler::Sample;
use rbrtcore::scene::Scene;
use rbrtcore::spectrum::Spectrum;

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

  pub fn from_paramset(params: &ParamSet) -> WhittedIntegrator {
    WhittedIntegrator::new(params.find_one_int("maxdepth", 5) as uint)
  }
}

impl SurfaceIntegrator for WhittedIntegrator {
  fn Li<'a>(&self, scene: &'a Scene<'a>, renderer: &Renderer, ray: &RayDifferential,
      intersection: &mut Intersection, sample: &Sample, rng: &mut TaskRng) -> Spectrum {
    // Evaluate bsdf at hit point
    let bsdf = intersection.get_bsdf(ray);

    // Initialize common variables for whitted integrator
    let p = bsdf.get_ref().dg_shading.p;
    let n = bsdf.get_ref().dg_shading.nn;
    let wo = -ray.ray.d;

    // Compute emitted light if ray hit an area light source
    let mut L = intersection.Le(&wo);

    // Add contribution of each light source
    for light in scene.lights.iter() {
      let mut wi = Vector::new(0.0, 0.0, 0.0);
      let mut pdf = 0.0;
      let mut visibility = VisibilityTester::new();
      let li = light.sample_l(&p, intersection.ray_epsilon,
        &LightSample::from_random(rng), ray.ray.time,
        &mut wi, &mut pdf, &mut visibility);

      if li.is_black() || pdf == 0.0 {
        continue;
      }

      let f = bsdf.get_ref().f(&wo, &wi, AllTypes);

      if !f.is_black() && visibility.unoccluded(scene) {
        L = L + f * li *
          (visibility.transmittance(scene, renderer, sample, rng) * (abs_dot(wi, n) / pdf));
      }
    }

    if ray.ray.depth + 1 < self.max_depth {
      // Trace rays for specular reflection and refraction
      L = L + specular_reflect(ray, bsdf.get_ref(), rng, intersection, renderer,
        scene, sample);
      L = L + specular_transmit(ray, bsdf.get_ref(), rng, intersection, renderer,
        scene, sample);
    }

    return L;
  }
}
