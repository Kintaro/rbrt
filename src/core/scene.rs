use geometry::{ BBox, Ray };
use primitive::Primitive;

pub struct Scene {
    aggregate: Box<Primitive>,
    bound:     BBox
}

impl Scene {
    pub fn intersect_p(&self, ray: &Ray) -> bool {
        self.aggregate.intersect_p(ray)
    }
}