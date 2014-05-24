use diffgeom::DifferentialGeometry;
use reflection::Bsdf;

pub trait Material {
  fn get_bsdf(&self, dg_geom: &DifferentialGeometry, dg_shading: &DifferentialGeometry) -> Bsdf;
  fn get_bssrdf(&self, dg_geom: &DifferentialGeometry, dg_shading: &DifferentialGeometry) -> Bsdf;
}
