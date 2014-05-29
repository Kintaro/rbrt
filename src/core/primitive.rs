use diffgeom::DifferentialGeometry;
use geometry::Ray;
use light::AreaLight;
use reflection::{ Bsdf, Bssrdf };
use transform::Transform;

pub trait Primitive {
  fn intersect_p(&self, ray: &Ray) -> bool;
  fn get_bsdf(&self, dg: &DifferentialGeometry, object_to_world: &Transform) -> Option<Bsdf>;
  fn get_bssrdf(&self, dg: &DifferentialGeometry, object_to_world: &Transform) -> Option<Bssrdf>;
  fn get_area_light(&self) -> Option<Box<AreaLight>>;
}
