use diffgeom::DifferentialGeometry;

pub trait Texture<T> {
  fn evaluate(&self, dg: &DifferentialGeometry) -> T;
}
