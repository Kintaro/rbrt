use diffgeom::DifferentialGeometry;
use geometry::Ray;
use reflection::Bsdf;
use transform::Transform;

pub trait Primitive {
    fn intersect_p(&self, ray: &Ray) -> bool;
    fn get_bsdf(&self, dg: &DifferentialGeometry, object_to_world: &Transform) -> Bsdf;
}