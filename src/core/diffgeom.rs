use std::rc::Rc;
use std::cell::RefCell;

use geometry::{ Normal, Point, RayDifferential, Vector, normalize, cross, dot, solve_linear_system };
use shape::Shape;

#[deriving(Clone)]
pub struct DifferentialGeometry {
  pub p:     Point,
  pub nn:    Normal,
  pub u:     f32,
  pub v:     f32,
  pub shape: Option<Rc<RefCell<Box<Shape>>>>,
  pub dpdu:  Vector,
  pub dpdv:  Vector,
  pub dndu:  Normal,
  pub dndv:  Normal,
  pub dpdx:  Vector,
  pub dpdy:  Vector,
  pub dudx:  f32,
  pub dvdx:  f32,
  pub dudy:  f32,
  pub dvdy:  f32
}

impl DifferentialGeometry {
  pub fn new(p: Point, dpdu: Vector, dpdv: Vector,
      dndu: Normal, dndv: Normal, u: f32, v: f32,
      sh: Option<Rc<RefCell<Box<Shape>>>>) -> DifferentialGeometry {
    let mut n = Normal::from_vector(&normalize(cross(dpdu, dpdv)));

    match sh {
      Some(ref x) => {
        if x.borrow().get_base().reverse_orientation ^ x.borrow().get_base().transform_swaps_handedness {
          n = -n;
        }
      },
      None => ()
    }

    DifferentialGeometry {
      p: p, nn: n, u: u, v: v, shape: sh,
      dpdu: dpdu, dpdv: dpdv, dndu: dndu, dndv: dndv,
      dpdx: Vector::zero(),
      dpdy: Vector::zero(),
      dudx: 0.0,
      dvdx: 0.0,
      dudy: 0.0,
      dvdy: 0.0
    }
  }
  pub fn compute_differentials(&mut self, ray: &RayDifferential) {
    if !ray.has_differentials {
      self.reset();
      return;
    }

    let d = -dot(self.nn, Vector::new(self.p.x, self.p.y, self.p.z));
    let rxv = Vector::new(ray.rx_origin.x, ray.rx_origin.y, ray.rx_origin.z);
    let tx = -(dot(self.nn, rxv) + d) / dot(self.nn, ray.rx_direction);

    if tx.is_nan() {
      self.reset();
      return;
    }

    let px = ray.rx_origin + ray.rx_direction * tx;
    let ryv = Vector::new(ray.ry_origin.x, ray.ry_origin.y, ray.ry_origin.z);
    let ty = -(dot(self.nn, ryv) + d) / dot(self.nn, ray.ry_direction);

    if ty.is_nan() {
      self.reset();
      return;
    }

    let py = ray.ry_origin + ray.ry_direction * ty;

    self.dpdx = px - self.p;
    self.dpdy = py - self.p;

    let mut a = [[0.0, ..2], ..2];
    let mut bx = [0.0, ..2];
    let mut by = [0.0, ..2];
    let mut axes = [0, ..2];

    if self.nn.x.abs() > self.nn.y.abs() && self.nn.x.abs() > self.nn.z.abs() {
      axes[0] = 1;
      axes[1] = 2;
    } else if self.nn.y.abs() > self.nn.z.abs() {
      axes[0] = 0;
      axes[1] = 2;
    } else {
      axes[0] = 0;
      axes[1] = 1;
    }

    a[0][0] = self.dpdu[axes[0]];
    a[0][1] = self.dpdu[axes[0]];
    a[1][0] = self.dpdu[axes[1]];
    a[1][1] = self.dpdu[axes[1]];

    bx[0] = px[axes[0]] - self.p[axes[0]];
    bx[1] = px[axes[1]] - self.p[axes[1]];
    by[0] = py[axes[0]] - self.p[axes[0]];
    by[1] = py[axes[1]] - self.p[axes[1]];

    if !solve_linear_system(a, bx, &mut self.dudx, &mut self.dvdx) {
      self.dudx = 0.0;
      self.dvdx = 0.0;
    }
    if !solve_linear_system(a, by, &mut self.dudy, &mut self.dvdy) {
      self.dudy = 0.0;
      self.dvdy = 0.0;
    }
  }

  pub fn reset(&mut self) {
    self.dudx = 0.0;
    self.dvdx = 0.0;
    self.dudy = 0.0;
    self.dvdy = 0.0;
    self.dpdx = Vector::zero();
    self.dpdy = Vector::zero();
  }

  pub fn reverse_orientation(&self) -> bool {
    match self.shape {
      None => false,
      Some(ref x) => x.borrow().get_base().reverse_orientation
    }
  }

  pub fn transform_swaps_handedness(&self) -> bool {
    match self.shape {
      None => false,
      Some(ref x) => x.borrow().get_base().transform_swaps_handedness
    }
  }
}
