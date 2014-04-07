use geometry::{ Point, Vector, RayDifferential, round_up_pow_2 };
use scene::Scene;
use spectrum::Spectrum;
use transform::Transform;

pub struct LightBase {
    num_samples: uint,
    light_to_world: Transform,
    world_to_light: Transform
}

pub trait Light {
    fn get_base(&self) -> LightBase;
    fn get_base_mut(&self) -> LightBase;

    fn is_delta_light(&self) -> bool;

    fn power(&self, scene: &Scene) -> Spectrum;
    fn Le(&self, ray: &RayDifferential) -> Spectrum;
    fn pdf(&self, p: &Point, wi: &Vector) -> f32;

    fn sh_project(&self, p: &Point, p_epsilon: f32, lmax: uint, scene: &Scene,
            compute_light_visibility: bool, time: f32, coeffs: &mut Vec<Spectrum>) {
        let ns = round_up_pow_2(self.get_base().num_samples);

        for i in range(0, ns) {

        }
    }
}