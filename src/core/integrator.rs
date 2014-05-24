extern crate rand;

use camera::Camera;
use geometry::RayDifferential;
use intersection::Intersection;
use renderer::Renderer;
use sampler::Sample;
use scene::Scene;
use spectrum::Spectrum;

use rand::Rng;

pub trait Integrator {
    fn preprocess(scene: &Scene, camera: &Camera) {}
    fn request_samples() {}
}

pub trait SurfaceIntegrator : Integrator {
    fn Li(&self, scene: &Scene, renderer: &Renderer, ray: &RayDifferential,
      intersection: &mut Intersection, sample: &Sample, rng: Box<Rng>) -> Spectrum;
}
