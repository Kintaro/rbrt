use std::rc::Rc;

use geometry::{ Normal, Point, RayDifferential, Vector, dot };
use shape::Shape;

pub struct DifferentialGeometry {
    p:     Point, 
    nn:    Normal,
    u:     f32,
    v:     f32,
    shape: Rc<~Shape>,
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

impl DifferentialGeometry {
    pub fn compute_differentials(&mut self, ray: &RayDifferential) {
        if !ray.has_differentials {
            self.reset();
            return;
        }

        let d = -dot(self.nn, Vector::new(self.p.x, self.p.y, self.p.z));
        let rxv = Vector::new(ray.rx_origin.x, ray.rx_origin.y, ray.rx_origin.z);
        let tx = -(dot(self.nn, rxv) + d) / dot(self.nn, ray.rx_direction);
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