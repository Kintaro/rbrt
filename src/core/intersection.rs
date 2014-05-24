use std::rc::Rc;

use diffgeom::DifferentialGeometry;
use geometry::{ RayDifferential, Vector };
use primitive::Primitive;
use reflection::Bsdf;
use spectrum::{ Spectrum, RgbSpectrum };
use transform::Transform;

pub struct Intersection<'a> {
  dg:        DifferentialGeometry<'a>,
  primitive:     Option<Rc<Box<Primitive>>>,
  world_to_object: Transform,
  object_to_world: Transform,
  shape_id:    uint,
  primitive_id:  uint,
  ray_epsilon:   f32
}

impl<'a> Intersection<'a> {
  pub fn get_bsdf(&mut self, ray: &RayDifferential) -> Bsdf {
    self.dg.compute_differentials(ray);
    self.primitive.get_mut_ref().get_bsdf(&self.dg, &self.object_to_world)
  }

  pub fn Le(&self, wo: &Vector) -> Spectrum {
    match self.primitive.get_ref().get_area_light() {
      Some(x) => x.L(&self.dg.p, &self.dg.nn, wo),
      None    => box RgbSpectrum::new(0.0)
    }
  }
}
