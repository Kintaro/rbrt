use std::rc::Rc;

use diffgeom::DifferentialGeometry;
use geometry::RayDifferential;
use primitive::Primitive;
use reflection::Bsdf;
use transform::Transform;

pub struct Intersection<'a> {
    dg:              DifferentialGeometry<'a>,
    primitive:       Option<Rc<Box<Primitive>>>,
    world_to_object: Transform,
    object_to_world: Transform,
    shape_id:        uint,
    primitive_id:    uint,
    ray_epsilon:     f32
}

impl<'a> Intersection<'a> {
    pub fn get_bsdf(&mut self, ray: &RayDifferential) -> Bsdf {
        self.dg.compute_differentials(ray);
        self.primitive.get_mut_ref().get_bsdf(&self.dg, &self.object_to_world)
    }
}