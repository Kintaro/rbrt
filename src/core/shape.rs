use transform::{ Applicable, Transform };
use geometry::{ Ray, BBox };

pub mod transform;
pub mod geometry;

pub struct ShapeBase {
    object_to_world: Transform,
    world_to_oject: Transform,
    shape_id: uint,
    next_shape_id: uint,
    reverse_orientation: bool, 
    transform_swaps_handedness: bool
}

pub trait Shape {
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