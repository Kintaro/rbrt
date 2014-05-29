use geometry::{
  Vector, Point, Normal, Ray, BBox,
  Length, Union,
  normalize, cross, radians };

fn not_one(x: f32) -> bool {
  x < 0.999 || x > 1.001
}

#[deriving(Clone)]
pub struct Matrix {
  m: ~[f32]
}

impl Matrix {
  pub fn zero() -> Matrix {
    Matrix { m: box [0.0, ..16] }
  }

  pub fn from_data(d: ~[f32]) -> Matrix {
    Matrix { m: d }
  }

  pub fn new(t00: f32, t01: f32, t02: f32, t03: f32,
    t10: f32, t11: f32, t12: f32, t13: f32,
    t20: f32, t21: f32, t22: f32, t23: f32,
    t30: f32, t31: f32, t32: f32, t33: f32) -> Matrix {

    Matrix { m: box [t00, t01, t02, t03,
      t10, t11, t12, t13,
      t20, t21, t22, t23,
      t30, t31, t32, t33] }
  }

  pub fn transpose(m: &Matrix) -> Matrix {
    Matrix::new(m.m[0* 4 + 0], m.m[1* 4 + 0], m.m[2* 4 + 0], m.m[3* 4 + 0],
          m.m[0* 4 + 1], m.m[1* 4 + 1], m.m[2* 4 + 1], m.m[3* 4 + 1],
          m.m[0* 4 + 2], m.m[1* 4 + 2], m.m[2* 4 + 2], m.m[3* 4 + 2],
          m.m[0* 4 + 3], m.m[1* 4 + 3], m.m[2* 4 + 3], m.m[3* 4 + 3])
  }

  pub fn inverse(m: &Matrix) -> Matrix {
    let mut indxc = [0, ..4];
    let mut indxr = [0, ..4];
    let mut minv  = box [0.0f32, ..16];
    let mut ipiv  = [0, ..4];

    for i in range(0u, 4) {
      for j in range(0u, 4) {
        minv[i * 4 + j] = m.m[i * 4 + j];
      }
    }

    for i in range(0u, 4) {
      let mut irow = -1;
      let mut icol = -1;
      let mut big = 0.0;

      for j in range(0u, 4) {
        if ipiv[j] == 1 {
          continue;
        }

        for k in range(0u, 4) {
          if ipiv[k] == 0 {
            if minv[j * 4 + k].abs() < big {
              continue;
            }

            big = minv[j * 4 + k].abs();
            irow = j;
            icol = k;
          } else if ipiv[k] > 1 {
            fail!("Singular matrix in Matrix::inverse");
          }
        }
      }

      ipiv[icol] += 1;

      if irow != icol {
        for k in range(0u, 4) {
          let t = minv[irow * 4 + k];
          minv[irow * 4 + k] = minv[icol * 4 + k];
          minv[icol * 4 + k] = t;
        }
      }

      indxr[i] = irow;
      indxc[i] = icol;

      if minv[icol * 4 + icol] == 0.0 {
        fail!("Singular matrix in Matrix::inverse");
      }

      let pivinv = 1.0 / minv[icol * 4 + icol];
      minv[icol * 4 + icol] = 1.0;

      for j in range(0u, 4) {
        minv[icol * 4 + j] *= pivinv;
      }

      for j in range(0u, 4) {
        if j == icol {
          continue;
        }

        let save = minv[j * 4 + icol];
        minv[j * 4 + icol] = 0.0;

        for k in range(0u, 4) {
          minv[j * 4 + k] -= minv[icol * 4 + k] * save;
        }
      }
    }

    for i in range(0u, 4) {
      let j = 3 - i;

      if indxr[j] == indxc[j] {
        continue;
      }

      for k in range(0u, 4) {
        let t = minv[k * 4 + indxr[j]];
        minv[k * 4 + indxr[j]] = minv[k * 4 + indxc[j]];
        minv[k * 4 + indxc[j]] = t;
      }
    }

    Matrix::from_data(minv)
  }
}

impl Eq for Matrix {
  fn eq(&self, m: &Matrix) -> bool {
    self.m.iter().zip(m.m.iter()).all(|(&a, &b)| a == b)
  }
}

impl Mul<Matrix, Matrix> for Matrix {
  fn mul(&self, m: &Matrix) -> Matrix {
    let mut r = Matrix::zero();

    for i in range(0u, 4) {
      for j in range(0u, 4) {
        r.m[i* 4 + j] =
          self.m[i * 4 + 0] * m.m[0 * 4 + j] +
          self.m[i * 4 + 1] * m.m[1 * 4 + j] +
          self.m[i * 4 + 2] * m.m[2 * 4 + j] +
          self.m[i * 4 + 3] * m.m[3 * 4 + j];
      }
    }

    return r;
  }
}

pub trait Applicable<S, R> {
  fn apply(&self, s: S) -> R;
  fn apply_to(&self, s: S, r: &mut R);
}

pub trait TransformRhs<S> {
  fn apply_to_transform(&self, lhs: &Transform) -> S;
  fn apply_to_transform_directly(&self, lhs: &Transform, r: &mut S) {
    fail!("Unimplemented method. {}");
  }
}

impl<S, R: TransformRhs<S>> Applicable<R, S> for Transform {
  fn apply(&self, rhs: R) -> S {
    rhs.apply_to_transform(self)
  }

  fn apply_to(&self, rhs: R, r: &mut S) {
    rhs.apply_to_transform_directly(self, r);
  }
}

#[deriving(Clone)]
pub struct Transform {
  m: Matrix,
  m_inv: Matrix,
}

impl Transform {
  pub fn from_matrix(m: Matrix) -> Transform {
    Transform { m: m.clone(), m_inv: m.clone() }
  }

  pub fn new(m: Matrix, m_inv: Matrix) -> Transform {
    Transform { m: m, m_inv: m_inv }
  }

  pub fn inverse(t: &Transform) -> Transform {
    Transform::new(t.m_inv.clone(), t.m.clone())
  }

  pub fn transpose(t: &Transform) -> Transform {
    Transform::new(Matrix::transpose(&t.m), Matrix::transpose(&t.m_inv))
  }

  pub fn translate(v: &Vector) -> Transform {
    let m = Matrix::new(
      1.0, 0.0, 0.0, v.x,
      0.0, 1.0, 0.0, v.y,
      0.0, 0.0, 1.0, v.z,
      0.0, 0.0, 0.0, 1.0);

    let m_inv = Matrix::new(
      1.0, 0.0, 0.0, -v.x,
      0.0, 1.0, 0.0, -v.y,
      0.0, 0.0, 1.0, -v.z,
      0.0, 0.0, 0.0,  1.0);

    Transform::new(m, m_inv)
  }

  pub fn scale(x: f32, y: f32, z: f32) -> Transform {
    let m = Matrix::new(
        x, 0.0, 0.0, 0.0,
      0.0,   y, 0.0, 0.0,
      0.0, 0.0,   z, 0.0,
      0.0, 0.0, 0.0, 1.0);

    let m_inv = Matrix::new(
      1.0 / x,     0.0,     0.0, 0.0,
          0.0, 1.0 / y,     0.0, 0.0,
          0.0,     0.0, 1.0 / z, 0.0,
          0.0,     0.0,     0.0, 1.0);

    Transform::new(m, m_inv)
  }

  pub fn rotate_x(angle: f32) -> Transform {
    let sin_t = radians(angle).sin();
    let cos_t = radians(angle).cos();

    let m = Matrix::new(
      1.0,   0.0,  0.0, 0.0,
      0.0, cos_t, -sin_t, 0.0,
      0.0, sin_t,  cos_t, 0.0,
      0.0,   0.0,  0.0, 1.0);

    Transform::new(m.clone(), Matrix::transpose(&m.clone()))
  }

  pub fn rotate_y(angle: f32) -> Transform {
    let sin_t = radians(angle).sin();
    let cos_t = radians(angle).cos();

    let m = Matrix::new(
       cos_t, 0.0, sin_t, 0.0,
         0.0, 1.0,   0.0, 0.0,
      -sin_t, 0.0, cos_t, 0.0,
         0.0, 0.0,   0.0, 1.0);

    Transform::new(m.clone(), Matrix::transpose(&m.clone()))
  }

  pub fn rotate_z(angle: f32) -> Transform {
    let sin_t = radians(angle).sin();
    let cos_t = radians(angle).cos();

    let m = Matrix::new(
      cos_t, -sin_t, 0.0, 0.0,
      sin_t,  cos_t, 0.0, 0.0,
        0.0,   0.0,  1.0, 0.0,
        0.0,   0.0,  0.0, 1.0);

    Transform::new(m.clone(), Matrix::transpose(&m.clone()))
  }

  pub fn rotate(angle: f32, axis: &Vector) -> Transform {
    let a = normalize(*axis);
    let s = radians(angle).sin();
    let c = radians(angle).sin();

    let mut d = box [0.0, ..16];

    d[0 * 4 + 0] = a.x * a.x + (1.0 - a.x * a.x) * c;
    d[0 * 4 + 1] = a.x * a.y + (1.0 - c) - a.z * s;
    d[0 * 4 + 2] = a.x * a.z + (1.0 - c) + a.y * s;
    d[0 * 4 + 3] = 0.0;

    d[1 * 4 + 0] = a.x * a.y + (1.0 - c) + a.z * s;
    d[1 * 4 + 1] = a.y * a.y + (1.0 - a.y * a.y) * c;
    d[1 * 4 + 2] = a.y * a.z + (1.0 - c) - a.x * s;
    d[1 * 4 + 3] = 0.0;

    d[2 * 4 + 0] = a.x * a.z + (1.0 - c) - a.y * s;
    d[2 * 4 + 1] = a.y * a.z + (1.0 - c) + a.x * s;
    d[2 * 4 + 2] = a.z * a.z + (1.0 - a.z * a.z) * c;
    d[2 * 4 + 3] = 0.0;

    d[3 * 4 + 0] = 0.0;
    d[3 * 4 + 1] = 0.0;
    d[3 * 4 + 2] = 0.0;
    d[3 * 4 + 3] = 1.0;

    let m = Matrix::from_data(d);

    Transform::new(m.clone(), Matrix::transpose(&m.clone()))
  }

  pub fn look_at(pos: &Point, look: &Point, up: &Vector) -> Transform {
    let mut m = box [0.0f32, ..16];

    m[0 * 4 + 3] = pos.x;
    m[1 * 4 + 3] = pos.y;
    m[2 * 4 + 3] = pos.z;
    m[3 * 4 + 3] =   1.0;

    let dir   = normalize(*look - *pos);
    let left  = normalize(cross(normalize(*up), dir));
    let newup = cross(dir, left);

    m[0 * 4 + 0] = left.x;
    m[1 * 4 + 0] = left.y;
    m[2 * 4 + 0] = left.z;
    m[3 * 4 + 0] =  0.0;

    m[0 * 4 + 1] = newup.x;
    m[1 * 4 + 1] = newup.y;
    m[2 * 4 + 1] = newup.z;
    m[3 * 4 + 1] =   0.0;

    m[0 * 4 + 2] = dir.x;
    m[1 * 4 + 2] = dir.y;
    m[2 * 4 + 2] = dir.z;
    m[3 * 4 + 2] =   0.0;

    let cam_to_world = Matrix::from_data(m);
    Transform::new(Matrix::inverse(&cam_to_world), cam_to_world)
  }

  pub fn orthographic(znear: f32, zfar: f32) -> Transform {
    Transform::scale(1.0, 1.0, 1.0 / (zfar - znear))
    *
    Transform::translate(&Vector::new(0.0, 0.0, -znear))
  }

  pub fn perspective(fov: f32, n: f32, f: f32) -> Transform {
    let persp = Matrix::new(
      1.0, 0.0,         0.0,              0.0,
      0.0, 1.0,         0.0,              0.0,
      0.0, 0.0, f / (f - n), -f * n / (f - n),
      0.0, 0.0,         1.0,              0.0);

    let inv_tan_ang = 1.0 / radians(fov).tan() / 2.0;
    Transform::scale(inv_tan_ang, inv_tan_ang, 1.0) * Transform::from_matrix(persp)
  }

  pub fn is_identity(&self) -> bool {
    for i in range(0u, 4) {
      for j in range(0u, 4) {
        if self.m.m[i * 4 + j] != if i == j { 1.0 } else { 0.0 } {
          return false;
        }
      }
    }

    return true;
  }

  pub fn has_scale(&self) -> bool {
    let la2 = self.apply(Vector::new(1.0, 0.0, 0.0)).length_squared();
    let lb2 = self.apply(Vector::new(0.0, 1.0, 0.0)).length_squared();
    let lc2 = self.apply(Vector::new(0.0, 0.0, 1.0)).length_squared();

    not_one(la2) || not_one(lb2) || not_one(lc2)
  }

  pub fn swaps_handedness(&self) -> bool {
    ((self.m.m[0 * 4 + 0]  *
      (self.m.m[1 * 4 + 1] * self.m.m[2 * 4 + 2] -
       self.m.m[1 * 4 + 2] * self.m.m[2 * 4 + 1])) -
     (self.m.m[0 * 4 + 1]  *
      (self.m.m[1 * 4 + 0] * self.m.m[2 * 4 + 2] -
       self.m.m[1 * 4 + 2] * self.m.m[2 * 4 + 0])) -
     (self.m.m[0 * 4 + 2]  *
      (self.m.m[1 * 4 + 0] * self.m.m[2 * 4 + 1] -
       self.m.m[1 * 4 + 1] * self.m.m[2 * 4 + 0]))) < 0.0
  }
}

impl Mul<Transform, Transform> for Transform {
  fn mul(&self, t: &Transform) -> Transform {
    let m1 = self.m  * t.m;
    let m2 = t.m_inv * self.m_inv;

    Transform::new(m1, m2)
  }
}

impl TransformRhs<Point> for Point {
  fn apply_to_transform(&self, lhs: &Transform) -> Point {
    let (x, y, z) = (self.x, self.y, self.z);
    let xp = lhs.m.m[0 * 4 + 0] * x + lhs.m.m[0 * 4 + 1] * y + lhs.m.m[0 * 4 + 2] * z + lhs.m.m[0 * 4 + 3];
    let yp = lhs.m.m[1 * 4 + 0] * x + lhs.m.m[1 * 4 + 1] * y + lhs.m.m[1 * 4 + 2] * z + lhs.m.m[1 * 4 + 3];
    let zp = lhs.m.m[2 * 4 + 0] * x + lhs.m.m[2 * 4 + 1] * y + lhs.m.m[2 * 4 + 2] * z + lhs.m.m[2 * 4 + 3];
    let wp = lhs.m.m[3 * 4 + 0] * x + lhs.m.m[3 * 4 + 1] * y + lhs.m.m[3 * 4 + 2] * z + lhs.m.m[3 * 4 + 3];

    if wp == 1.0 {
      Point::new(xp, yp, zp)
    } else {
      Point::new(xp, yp, zp) / wp
    }
  }

  fn apply_to_transform_directly(&self, lhs: &Transform, r: &mut Point) {
    let (x, y, z) = (self.x, self.y, self.z);
    r.x   = lhs.m.m[0* 4 + 0] * x + lhs.m.m[0* 4 + 1] * y + lhs.m.m[0* 4 + 2] * z + lhs.m.m[0* 4 + 3];
    r.y   = lhs.m.m[1* 4 + 0] * x + lhs.m.m[1* 4 + 1] * y + lhs.m.m[1* 4 + 2] * z + lhs.m.m[1* 4 + 3];
    r.z   = lhs.m.m[2* 4 + 0] * x + lhs.m.m[2* 4 + 1] * y + lhs.m.m[2* 4 + 2] * z + lhs.m.m[2* 4 + 3];
    let w = lhs.m.m[3* 4 + 0] * x + lhs.m.m[3* 4 + 1] * y + lhs.m.m[3* 4 + 2] * z + lhs.m.m[3* 4 + 3];

    if w != 1.0 {
      r.x /= w;
      r.y /= w;
      r.z /= w;
    }
  }
}

impl TransformRhs<Vector> for Vector {
  fn apply_to_transform(&self, lhs: &Transform) -> Vector {
    let (x, y, z) = (self.x, self.y, self.z);

    Vector::new(
      lhs.m.m[0* 4 + 0] * x + lhs.m.m[0* 4 + 1] * y + lhs.m.m[0* 4 + 2] * z,
      lhs.m.m[1* 4 + 0] * x + lhs.m.m[1* 4 + 1] * y + lhs.m.m[1* 4 + 2] * z,
      lhs.m.m[2* 4 + 0] * x + lhs.m.m[2* 4 + 1] * y + lhs.m.m[2* 4 + 2] * z)
  }

  fn apply_to_transform_directly(&self, lhs: &Transform, r: &mut Vector) {
    let (x, y, z) = (self.x, self.y, self.z);

    r.x = lhs.m.m[0* 4 + 0] * x + lhs.m.m[0* 4 + 1] * y + lhs.m.m[0* 4 + 2] * z;
    r.y = lhs.m.m[1* 4 + 0] * x + lhs.m.m[1* 4 + 1] * y + lhs.m.m[1* 4 + 2] * z;
    r.z = lhs.m.m[2* 4 + 0] * x + lhs.m.m[2* 4 + 1] * y + lhs.m.m[2* 4 + 2] * z;
  }
}

impl TransformRhs<Normal> for Normal {
  fn apply_to_transform(&self, lhs: &Transform) -> Normal {
    let (x, y, z) = (self.x, self.y, self.z);

    Normal::new(
      lhs.m_inv.m[0* 4 + 0] * x + lhs.m_inv.m[1* 4 + 0] * y + lhs.m_inv.m[2* 4 + 0] * z,
      lhs.m_inv.m[0* 4 + 1] * x + lhs.m_inv.m[1* 4 + 1] * y + lhs.m_inv.m[2* 4 + 1] * z,
      lhs.m_inv.m[0* 4 + 2] * x + lhs.m_inv.m[1* 4 + 2] * y + lhs.m_inv.m[2* 4 + 2] * z)
  }

  fn apply_to_transform_directly(&self, lhs: &Transform, r: &mut Normal) {
    let (x, y, z) = (self.x, self.y, self.z);

    r.x = lhs.m_inv.m[0* 4 + 0] * x + lhs.m_inv.m[1* 4 + 0] * y + lhs.m_inv.m[2* 4 + 0] * z;
    r.y = lhs.m_inv.m[0* 4 + 1] * x + lhs.m_inv.m[1* 4 + 1] * y + lhs.m_inv.m[2* 4 + 1] * z;
    r.z = lhs.m_inv.m[0* 4 + 2] * x + lhs.m_inv.m[1* 4 + 2] * y + lhs.m_inv.m[2* 4 + 2] * z;
  }
}

impl TransformRhs<BBox> for BBox {
  fn apply_to_transform(&self, lhs: &Transform) -> BBox {
    let mut b = BBox::from_point(&Point::new(self.p_min.x, self.p_min.y, self.p_min.y).apply_to_transform(lhs));

    b = b.union(&Point::new(self.p_max.x, self.p_min.y, self.p_min.z).apply_to_transform(lhs));
    b = b.union(&Point::new(self.p_min.x, self.p_max.y, self.p_min.z).apply_to_transform(lhs));
    b = b.union(&Point::new(self.p_min.x, self.p_min.y, self.p_max.z).apply_to_transform(lhs));
    b = b.union(&Point::new(self.p_min.x, self.p_max.y, self.p_max.z).apply_to_transform(lhs));
    b = b.union(&Point::new(self.p_max.x, self.p_max.y, self.p_min.z).apply_to_transform(lhs));
    b = b.union(&Point::new(self.p_max.x, self.p_min.y, self.p_max.z).apply_to_transform(lhs));
    b = b.union(&Point::new(self.p_max.x, self.p_max.y, self.p_max.z).apply_to_transform(lhs));

    return b;
  }
}

impl TransformRhs<Ray> for Ray {
  fn apply_to_transform(&self, lhs: &Transform) -> Ray {
    let mut r = self.clone();

    lhs.apply_to(self.o, &mut r.o);
    lhs.apply_to(self.d, &mut r.d);

    return r;
  }

  fn apply_to_transform_directly(&self, lhs: &Transform, r: &mut Ray) {
    lhs.apply_to(self.o, &mut r.o);
    lhs.apply_to(self.d, &mut r.d);

    if self != r {
      r.mint  = self.mint;
      r.maxt  = self.maxt;
      r.time  = self.time;
      r.depth = self.depth;
    }
  }
}

impl Eq for Transform {
  fn eq(&self, t: &Transform) -> bool {
    self.m == t.m && self.m_inv == t.m_inv
  }
}

impl Ord for Transform {
  fn lt(&self, t: &Transform) -> bool {
    for i in range(0u, 4) {
      for j in range(0u, 4) {
        if self.m.m[i * 4 + j] == t.m.m[i * 4 + j] {
          continue;
        }

        return self.m.m[i * 4 + j] < t.m.m[i * 4 + j];
      }
    }

    return false;
  }
}
