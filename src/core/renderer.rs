use geometry::{ RayDifferential };
use sampler::{ Sample };
use scene::{ Scene };
use spectrum::{ Spectrum };

pub trait Renderer {
    fn render(&self, scene: &Scene);
    fn Li(&self, scene: &Scene, ray: &RayDifferential, sample: &Sample) -> Spectrum;
    fn transmittance(&self, scene: &Scene, ray: &RayDifferential) -> Spectrum;
}