use geometry::{ clamp, mod_t };

pub enum ImageWrap {
  Repeat,
  Black,
  Clamp
}

pub struct ResampleWeight {
  pub first_texel: uint,
  pub weight: [f32, ..4]
}

pub trait MipMapType {
  fn usize(&self) -> uint;
  fn vsize(&self) -> uint;
}

pub struct MipMap<T> {
  pub do_trilinear: bool,
  pub max_anisotropy: f32,
  pub wrap_mode: ImageWrap,
  pub width: uint,
  pub height: uint,
  pub num_levels: uint,
  pub pyramid: ~[T]
}

impl<T: MipMapType> MipMap<T> {
  pub fn texel(&self, level: uint, s: uint, t: uint) -> T {
    let l = &self.pyramid[level];
    let (ss, tt) = match self.wrap_mode {
      Repeat => (mod_t(s, l.usize()), mod_t(t, l.vsize())),
      Clamp  => (clamp(s, 0, l.usize() - 1), clamp(t, 0, l.vsize() - 1)),
      Black  => fail!("not implemented")
    };

    fail!("not implemented");
  }

  pub fn lookup_w(&self, s: f32, t: f32, w: f32) -> T {
    fail!("not implemented");
  }

  pub fn lookup(&self, s: f32, t: f32, ds0: f32, dt0: f32, ds1: f32, dt1: f32) -> T {
    fail!("not implemented");
  }

  fn triangle(&self, level: uint, s: f32, t: f32) -> T {
    fail!("not implemented");
  }

  fn ewa(&self, level: uint, s: f32, t: f32, ds0: f32, dt0: f32, ds1: f32, dt1: f32) -> T {
    fail!("not implemented");
  }
}
