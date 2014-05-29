use diffgeom::DifferentialGeometry;
use geometry::{ Vector, Normal, cross, cross_n, normalize, face_forward };
use reflection::Bsdf;
use texture::Texture;

pub trait Material {
  fn get_bsdf(&self, dg_geom: &DifferentialGeometry, dg_shading: &DifferentialGeometry) -> Bsdf;
  fn get_bssrdf(&self, dg_geom: &DifferentialGeometry, dg_shading: &DifferentialGeometry) -> Option<Bsdf> {
    None
  }
}

pub fn bump(d: &Texture<f32>, dg_geom: &DifferentialGeometry,
    dg_shading: DifferentialGeometry) -> DifferentialGeometry {
  // Compute offset positions and evaluate displacement
  let mut dg_eval = dg_shading.clone();

  let mut du = 0.5 * dg_shading.dudx.abs() + dg_shading.dudy.abs();
  if du == 0.0 {
    du = 0.01;
  }

  dg_eval.p  = dg_shading.p + dg_shading.dpdu * du;
  dg_eval.u  = dg_shading.u + du;
  dg_eval.nn = normalize(cross_n(dg_shading.dpdu, dg_shading.dpdv) + dg_shading.dndu * du);

  let u_displace = d.evaluate(&dg_eval);

  let mut dv = 0.5 * dg_shading.dvdx.abs() + dg_shading.dvdy.abs();
  if dv == 0.0 {
    dv = 0.01;
  }

  dg_eval.p = dg_shading.p + dg_shading.dpdv * dv;
  dg_eval.u = dg_shading.u;
  dg_eval.v = dg_shading.v + dv;
  dg_eval.nn = normalize(cross_n(dg_shading.dpdu, dg_shading.dpdv) + dg_shading.dndv * dv);

  let v_displace = d.evaluate(&dg_eval);
  let displace = d.evaluate(&dg_shading);

  // Compute bump-mapped differential geometry
  let mut dg_bump = dg_shading.clone();
  dg_bump.dpdu = dg_shading.dpdu + Vector::from_normal(&dg_shading.nn) * (u_displace - displace) / du +
    Vector::from_normal(&dg_shading.dndu) * displace;
  dg_bump.dpdv = dg_shading.dpdv + Vector::from_normal(&dg_shading.nn) * (v_displace - displace) / dv +
    Vector::from_normal(&dg_shading.dndv) * displace;
  dg_bump.nn = Normal::from_vector(&normalize(cross(dg_bump.dpdu, dg_bump.dpdv)));

  let x = dg_bump.reverse_orientation();
  let y = dg_bump.transform_swaps_handedness();

  if x ^ y {
    dg_bump.nn = -dg_bump.nn;
  }

  // Orient shading normal to match geometric normal
  dg_bump.nn = face_forward(dg_bump.nn, dg_geom.nn);

  return dg_bump;
}
