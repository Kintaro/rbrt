use transform::{ Applicable, Transform };
use geometry::{ Ray, BBox };

pub struct ShapeBase {
  object_to_world: Transform,
  world_to_oject: Transform,
  shape_id: uint,
  next_shape_id: uint,
  reverse_orientation: bool,
  transform_swaps_handedness: bool
}

pub trait Shape<'a> {
  fn get_base(&'a self) -> ShapeBase;
  fn get_base_mut(&'a mut self) -> &'a mut ShapeBase;
  fn object_bound(&'a self) -> BBox;
  fn area(&'a self) -> f32;
  fn intersect_p(&'a self, ray: &Ray) -> bool;

  fn world_bound(&'a self) -> BBox {
    self.get_base().object_to_world.apply(self.object_bound())
  }

  fn pdf(&'a self) -> f32 {
    1.0 / self.area()
  }

  fn can_intersect(&'a self) -> bool { true }
}
