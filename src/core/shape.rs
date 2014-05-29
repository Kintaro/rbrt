use transform::{ Applicable, Transform };
use geometry::{ Ray, BBox };

#[deriving(Clone)]
pub struct ShapeBase {
  pub object_to_world: Transform,
  pub world_to_oject: Transform,
  pub shape_id: uint,
  pub next_shape_id: uint,
  pub reverse_orientation: bool,
  pub transform_swaps_handedness: bool
}

pub trait Shape : Clone {
  fn get_base(&self) -> ShapeBase;
  fn object_bound(&self) -> BBox;
  fn area(&self) -> f32;
  fn intersect_p(&self, ray: &Ray) -> bool;

  fn world_bound(&self) -> BBox {
    self.get_base().object_to_world.apply(self.object_bound())
  }

  fn pdf(&self) -> f32 {
    1.0 / self.area()
  }

  fn can_intersect(&self) -> bool { true }
}
