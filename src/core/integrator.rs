use camera::Camera;
use geometry::RayDifferential;
use renderer::Renderer;
use scene::Scene;
use spectrum::Spectrum;

pub trait Integrator {
    fn preprocess(scene: &Scene, camera: &Camera);
    fn request_samples();
}

pub trait SurfaceIntegrator : Integrator {
    fn Li(&self, scene: &Scene, renderer: &Renderer, ray: &RayDifferential) -> Spectrum;
}