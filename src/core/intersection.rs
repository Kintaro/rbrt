use std::rc::Rc;
use std::cell::RefCell;

use diffgeom::DifferentialGeometry;
use geometry::{ RayDifferential, Vector };
use primitive::Primitive;
use reflection::{ Bsdf, Bssrdf };
use spectrum::Spectrum;
use transform::Transform;

pub struct Intersection {
  pub dg:              DifferentialGeometry,
  pub primitive:       Option<Rc<RefCell<Box<Primitive>>>>,
  pub world_to_object: Transform,
  pub object_to_world: Transform,
  pub shape_id:        uint,
  pub primitive_id:    uint,
  pub ray_epsilon:     f32
}

impl Intersection {
  pub fn get_bsdf(&mut self, ray: &RayDifferential) -> Option<Bsdf> {
    self.dg.compute_differentials(ray);
    self.primitive.get_ref().borrow().get_bsdf(&self.dg, &self.object_to_world)
  }

  pub fn get_bssrdf(&mut self, ray: &RayDifferential) -> Option<Bssrdf> {
    self.dg.compute_differentials(ray);
    self.primitive.get_ref().borrow().get_bssrdf(&self.dg, &self.object_to_world)
  }

  pub fn Le(&self, wo: &Vector) -> Spectrum {
    match self.primitive.get_ref().borrow().get_area_light() {
      Some(x) => x.L(&self.dg.p, &self.dg.nn, wo),
      None    => Spectrum::new(0.0)
    }
  }
}
