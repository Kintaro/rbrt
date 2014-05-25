use std::f32;

pub trait Length {
  fn length_squared(&self) -> f32;

  fn length(&self) -> f32 {
    self.length_squared().sqrt()
  }
}

#[deriving(Show, Eq, Clone)]
pub struct Vector {
  pub x: f32,
  pub y: f32,
  pub z: f32
}

impl Vector {
  pub fn zero() -> Vector {
    Vector { x: 0.0, y: 0.0, z: 0.0 }
  }

  pub fn new(x: f32, y: f32, z: f32) -> Vector {
    Vector { x: x, y: y, z: z }
  }
}

impl Length for Vector {
  fn length_squared(&self) -> f32 {
    self.x * self.x + self.y * self.y + self.z * self.z
  }
}

impl Add<Vector, Vector> for Vector {
  fn add(&self, rhs: &Vector) -> Vector {
    Vector::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
  }
}

impl Sub<Vector, Vector> for Vector {
  fn sub(&self, rhs: &Vector) -> Vector {
    Vector::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
  }
}

impl Mul<f32, Vector> for Vector {
  fn mul(&self, rhs: &f32) -> Vector {
    Vector::new(self.x * *rhs, self.y * *rhs, self.z * *rhs)
  }
}

impl Div<f32, Vector> for Vector {
  fn div(&self, rhs: &f32) -> Vector {
    Vector::new(self.x / *rhs, self.y / *rhs, self.z / *rhs)
  }
}

impl Neg<Vector> for Vector {
  fn neg(&self) -> Vector {
    Vector::new(-self.x, -self.y, -self.z)
  }
}

impl Index<uint, f32> for Vector {
  fn index(&self, index: &uint) -> f32 {
    match index {
      &0 => self.x,
      &1 => self.y,
      &2 => self.z,
      _  => fail!("Unknown vector index")
    }
  }
}

#[deriving(Show, Eq, Clone)]
pub struct Point {
  pub x: f32,
  pub y: f32,
  pub z: f32
}

impl Point {
  pub fn zero() -> Point {
    Point { x: 0.0, y: 0.0, z: 0.0 }
  }

  pub fn new(x: f32, y: f32, z: f32) -> Point {
    Point { x: x, y: y, z: z }
  }
}

pub trait PointRhsAdd<S> {
  fn add_to_point(&self, lhs: &Point) -> S;
}
pub trait PointRhsSub<S> {
  fn sub_from_point(&self, lhs: &Point) -> S;
}

impl<S, R: PointRhsAdd<S>> Add<R, S> for Point {
  fn add(&self, rhs: &R) -> S {
    rhs.add_to_point(self)
  }
}

impl<S, R: PointRhsSub<S>> Sub<R, S> for Point {
  fn sub(&self, rhs: &R) -> S {
    rhs.sub_from_point(self)
  }
}

impl PointRhsAdd<Point> for Point {
  fn add_to_point(&self, lhs: &Point) -> Point {
    Point::new(lhs.x + self.x, lhs.y + self.y, lhs.z + self.z)
  }
}
impl PointRhsSub<Vector> for Point {
  fn sub_from_point(&self, lhs: &Point) -> Vector {
    Vector::new(lhs.x - self.x, lhs.y - self.y, lhs.z - self.z)
  }
}

impl PointRhsAdd<Point> for Vector {
  fn add_to_point(&self, lhs: &Point) -> Point {
    Point::new(lhs.x + self.x, lhs.y + self.y, lhs.z + self.z)
  }
}
impl PointRhsSub<Point> for Vector {
  fn sub_from_point(&self, lhs: &Point) -> Point {
    Point::new(lhs.x - self.x, lhs.y - self.y, lhs.z - self.z)
  }
}

impl Mul<f32, Point> for Point {
  fn mul(&self, rhs: &f32) -> Point {
    Point::new(self.x * *rhs, self.y * *rhs, self.z * *rhs)
  }
}

impl Div<f32, Point> for Point {
  fn div(&self, rhs: &f32) -> Point {
    Point::new(self.x / *rhs, self.y / *rhs, self.z / *rhs)
  }
}

impl Index<uint, f32> for Point {
  fn index(&self, index: &uint) -> f32 {
    match index {
      &0 => self.x,
      &1 => self.y,
      &2 => self.z,
      _  => fail!("Unknown point index")
    }
  }
}

#[deriving(Show, Eq, Clone)]
pub struct Normal {
  pub x: f32,
  pub y: f32,
  pub z: f32
}

impl Normal {
  pub fn zero() -> Normal {
    Normal { x: 0.0, y: 0.0, z: 0.0 }
  }

  pub fn new(x: f32, y: f32, z: f32) -> Normal {
    Normal { x: x, y: y, z: z }
  }
}

impl Length for Normal {
  fn length_squared(&self) -> f32 {
    self.x * self.x + self.y * self.y + self.z * self.z
  }
}

impl Add<Normal, Normal> for Normal {
  fn add(&self, rhs: &Normal) -> Normal {
    Normal::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
  }
}

impl Sub<Normal, Normal> for Normal {
  fn sub(&self, rhs: &Normal) -> Normal {
    Normal::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
  }
}

impl Mul<f32, Normal> for Normal {
  fn mul(&self, rhs: &f32) -> Normal {
    Normal::new(self.x * *rhs, self.y * *rhs, self.z * *rhs)
  }
}

impl Div<f32, Normal> for Normal {
  fn div(&self, rhs: &f32) -> Normal {
    Normal::new(self.x / *rhs, self.y / *rhs, self.z / *rhs)
  }
}

impl Index<uint, f32> for Normal {
  fn index(&self, index: &uint) -> f32 {
    match index {
      &0 => self.x,
      &1 => self.y,
      &2 => self.z,
      _  => fail!("Unknown normal index")
    }
  }
}

#[deriving(Eq, Clone)]
pub struct Ray {
  pub o: Point,
  pub d: Vector,
  pub mint: f32,
  pub maxt: f32,
  pub time: f32,
  pub depth: uint
}

impl Ray {
  pub fn zero() -> Ray {
    Ray {
      o:   Point::zero(),
      d:   Vector::zero(),
      mint:  0.0,
      maxt:  f32::INFINITY,
      time:  0.0,
      depth: 0
    }
  }

  pub fn apply(&self, t: f32) -> Point {
    self.o + self.d * t
  }
}

pub struct RayDifferential {
  pub ray: Ray,
  pub has_differentials: bool,
  pub rx_origin: Point,
  pub ry_origin: Point,
  pub rx_direction: Vector,
  pub ry_direction: Vector
}

impl RayDifferential {
  pub fn new(r: &Ray) -> RayDifferential {
    fail!("not implemented");
  }

  pub fn apply(&self, t: f32) -> Point {
    self.ray.apply(t)
  }

  pub fn scale_differentials(&mut self, s: f32) {
    self.rx_origin  = self.ray.o + (self.rx_origin  - self.ray.o) * s;
    self.ry_origin  = self.ray.o + (self.ry_origin  - self.ray.o) * s;
    self.rx_direction = self.ray.d + (self.rx_direction - self.ray.d) * s;
    self.ry_direction = self.ray.d + (self.ry_direction - self.ray.d) * s;
  }
}

#[deriving(Show, Eq, Clone)]
pub struct BBox {
  pub p_min: Point,
  pub p_max: Point
}

pub trait BBoxRhs<S> {
  fn box_union(&self, lhs: &BBox) -> S;
}

pub trait Union<R, S> {
  fn union(&self, rhs: &R) -> S;
}

impl<S, R: BBoxRhs<S>> Union<R, S> for BBox {
  fn union(&self, rhs: &R) -> S {
    rhs.box_union(self)
  }
}

impl BBoxRhs<BBox> for Point {
  fn box_union(&self, lhs: &BBox) -> BBox {
    let mut b = lhs.clone();

    b.p_min.x = b.p_min.x.min(self.x);
    b.p_min.y = b.p_min.y.min(self.y);
    b.p_min.z = b.p_min.z.min(self.z);
    b.p_max.x = b.p_max.x.max(self.x);
    b.p_max.y = b.p_max.y.max(self.y);
    b.p_max.z = b.p_max.z.max(self.z);

    return b;
  }
}

impl BBoxRhs<BBox> for BBox {
  fn box_union(&self, lhs: &BBox) -> BBox {
    let mut b = lhs.clone();

    b.p_min.x = b.p_min.x.min(self.p_min.x);
    b.p_min.y = b.p_min.y.min(self.p_min.y);
    b.p_min.z = b.p_min.z.min(self.p_min.z);
    b.p_max.x = b.p_max.x.max(self.p_max.x);
    b.p_max.y = b.p_max.y.max(self.p_max.y);
    b.p_max.z = b.p_max.z.max(self.p_max.z);

    return b;
  }
}

impl BBox {
  pub fn from_point(p: &Point) -> BBox {
    BBox::new(p, p)
  }

  pub fn new(min: &Point, max: &Point) -> BBox {
    BBox { p_min: *min, p_max: *max }
  }

  pub fn inside(&self, pt: &Point) -> bool {
    pt.x >= self.p_min.x && pt.x <= self.p_max.x &&
    pt.y >= self.p_min.y && pt.y <= self.p_max.y &&
    pt.z >= self.p_min.z && pt.z <= self.p_max.z
  }

  pub fn expand(&mut self, delta: f32) {
    self.p_min = self.p_min - Vector::new(delta, delta, delta);
    self.p_max = self.p_max + Vector::new(delta, delta, delta);
  }

  pub fn surface_area(&self) -> f32 {
    let d = self.p_max - self.p_min;
    2.0 * (d.x * d.y + d.x * d.z + d.y * d.z)
  }

  pub fn volume(&self) -> f32 {
    let d = self.p_max - self.p_min;
    d.x * d.y * d.z
  }

  pub fn intersect_p(&self, ray: &Ray) -> Option<(f32, f32)> {
    let mut t0 = ray.mint;
    let mut t1 = ray.maxt;

    for i in range(0u, 3) {
      let inverted_ray_dir = 1.0 / ray.d[i];
      let tnear = (self.p_min[i] - ray.o[i]) * inverted_ray_dir;
      let tfar  = (self.p_max[i] - ray.o[i]) * inverted_ray_dir;

      let (a, b) = if tnear > tfar {
        (tfar, tnear)
      } else {
        (tnear, tfar)
      };

      t0 = a.max(t0);
      t1 = b.min(t1);

      if t0 > t1 {
        return None;
      }
    }

    Some((t0, t1))
  }

  pub fn maximum_extent(&self) -> uint {
    let diag = self.p_max - self.p_min;

    match (diag.x > diag.y, diag.x > diag.z, diag.y > diag.z) {
      (true, true,  _) => 0,
      (   _,  _, true) => 1,
      _          => 2
    }
  }
}

/// Utility methods

pub fn clamp<T: Ord>(val: T, low: T, high: T) -> T {
  if val < low {
    low
  } else if val > high {
    high
  } else {
    val
  }
}

pub fn lerp(t: f32, v1: f32, v2: f32) -> f32 {
  (1.0 - t) * v1 + t * v2
}

pub fn radians(deg: f32) -> f32 {
  (f32::consts::PI / 180.0) * deg
}

pub fn degrees(rad: f32) -> f32 {
  (180.0 / f32::consts::PI) * rad
}

pub fn is_power_of_2(v: uint) -> bool {
  v != 0 && (v & (v - 1)) == 0
}

pub fn round_up_pow_2(v: uint) -> uint {
  let mut x = v - 1;
  x |= x >>  1; x |= x >> 2;
  x |= x >>  4; x |= x >> 8;
  x |= x >> 16;
  x + 1
}

pub fn quadratic(a: f32, b: f32, c: f32) -> Option<(f32, f32)> {
  let discrim = b * b - 4.0 * a * c;
  if discrim < 0.0 {
    return None;
  }

  let root_discrim = discrim.sqrt();

  let q = if b < 0.0 {
    -0.5 * (b - root_discrim)
  } else {
    -0.5 * (b + root_discrim)
  };

  let t0 = q / a;
  let t1 = c / q;

  if t0 > t1 {
    Some((t1, t0))
  } else {
    Some((t0, t1))
  }
}

/// Normalize the object (vector, normal, ...) to unit length 1.0
pub fn normalize<T: Length + Div<f32, T>>(x: T) -> T {
  x / x.length()
}

/// Compute the distance between two points
pub fn distance(a: &Point, b: &Point) -> f32 {
  (a - *b).length()
}

/// Compute the squared distance between two points
pub fn distance_squared(a: &Point, b: &Point) -> f32 {
  (a - *b).length_squared()
}

/// Compute the dot product between two vectors or normals
pub fn dot<T: Index<uint, f32> + Length, S: Index<uint, f32> + Length>(a: T, b: S) -> f32 {
  a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

/// Compute the absolute dot product between
/// two vectors or normals
pub fn abs_dot<T: Index<uint, f32> + Length, S: Index<uint, f32> + Length>(a: T, b: S) -> f32 {
  (a[0] * b[0] + a[1] * b[1] + a[2] * b[2]).abs()
}

/// Compute the cross product between two vector or normals
pub fn cross<T: Index<uint, f32> + Length, S: Index<uint, f32> + Length>(a: T, b: S) -> Vector {
  let (v1x, v1y, v1z) = (a[0], a[1], a[2]);
  let (v2x, v2y, v2z) = (b[0], b[1], b[2]);
  Vector::new((v1y * v2z) - (v1z * v2y),
    (v1z * v2x) - (v1x * v2z),
    (v1x * v2y) - (v1y * v2x))
}

pub fn spherical_direction(sin_theta: f32, cos_theta: f32, phi: f32) -> Vector {
  Vector::new(sin_theta * phi.cos(),
    sin_theta * phi.sin(),
    cos_theta)
}

pub fn spherical_theta(v: &Vector) -> f32 {
  clamp(v.z, -1.0, 1.0).acos()
}

pub fn spherical_phi(v: &Vector) -> f32 {
  let p = v.y.atan2(v.x);
  if p < 0.0 {
    p + 2.0 * 3.142
  } else {
    p
  }
}

pub fn solve_linear_system(a: [[f32, ..2], ..2], b: [f32, ..2], x0: &mut f32, x1: &mut f32) -> bool {
  let det = a[0][0] * a[1][1] - a[0][1] * a[1][0];
  if det.abs() < 1e-10 {
    return false;
  }

  *x0 = (a[1][1] * b[0] - a[0][1] * b[1]) / det;
  *x1 = (a[0][0] * b[1] - a[1][0] * b[0]) / det;

  !x0.is_nan() && !x1.is_nan()
}
