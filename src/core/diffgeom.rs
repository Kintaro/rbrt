use std::rc::Rc;

use geometry::{ Normal, Point, RayDifferential, Vector, dot, solve_linear_system };
use shape::Shape;

pub struct DifferentialGeometry<'a> {
    p:     Point,
    nn:    Normal,
    u:     f32,
    v:     f32,
    shape: Rc<Box<Shape<'a>>>,
    dpdu:  Vector,
    dpdv:  Vector,
    dndu:  Normal,
    dndv:  Normal,
    dpdx:  Vector,
    dpdy:  Vector,
    dudx:  f32,
    dvdx:  f32,
    dudy:  f32,
    dvdy:  f32
}

impl<'a> DifferentialGeometry<'a> {
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

    fn reset(&mut self) {
        self.dudx = 0.0;
        self.dvdx = 0.0;
        self.dudy = 0.0;
        self.dvdy = 0.0;
        self.dpdx = Vector::zero();
        self.dpdy = Vector::zero();
    }
}
