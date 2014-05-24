use geometry::{ BBox, Ray };
use light::Light;
use primitive::Primitive;

pub struct Scene<'a> {
  pub aggregate: Box<Primitive>,
  pub bound:   BBox,
  pub lights:  Vec<Box<Light<'a>>>
}

impl<'a> Scene<'a> {
  pub fn intersect_p(&self, ray: &Ray) -> bool {
    self.aggregate.intersect_p(ray)
  }
}
