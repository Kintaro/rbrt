pub fn xyz_to_rgb(xyz: &[f32, ..3], rgb: &mut [f32, ..3]) {
  rgb[0] =  3.240479 * xyz[0] - 1.537150 * xyz[1] - 0.498535 * xyz[2];
  rgb[1] = -0.969256 * xyz[0] + 1.875991 * xyz[1] + 0.041556 * xyz[2];
  rgb[2] =  0.055648 * xyz[0] - 0.204043 * xyz[1] + 1.057311 * xyz[2];
}

pub fn rgb_to_xyz(rgb: &[f32, ..3], xyz: &mut [f32, ..3]) {
  xyz[0] = 0.412453 * rgb[0] + 0.357580 * rgb[1] + 0.180423 * rgb[2];
  xyz[1] = 0.212671 * rgb[0] + 0.715160 * rgb[1] + 0.072169 * rgb[2];
  xyz[2] = 0.019334 * rgb[0] + 0.119193 * rgb[1] + 0.950227 * rgb[2];
}


pub enum SpectrumType {
  Reflectance,
  Illuminant
}

pub struct SpectrumBase;

pub trait CoefficientSpectrum {

}

#[deriving(Clone)]
pub struct Spectrum;

impl Spectrum {
  pub fn new(v: f32) -> Spectrum {
    Spectrum
  }

  pub fn is_black(&self) -> bool {
    fail!("not implemented");
  }
}

impl Add<Spectrum, Spectrum> for Spectrum {
  fn add(&self, rhs: &Spectrum) -> Spectrum {
    fail!("not implemented");
  }
}

pub trait SpectrumRhsMul<S> {
  fn mul_with_spectrum(&self, lhs: &Spectrum) -> S;
}

impl<S, R: SpectrumRhsMul<S>> Mul<R, S> for Spectrum {
  fn mul(&self, rhs: &R) -> S {
    rhs.mul_with_spectrum(self)
  }
}

impl SpectrumRhsMul<Spectrum> for f32 {
  fn mul_with_spectrum(&self, lhs: &Spectrum) -> Spectrum {
    Spectrum::new(0.0)
  }
}

impl SpectrumRhsMul<Spectrum> for Spectrum {
  fn mul_with_spectrum(&self, lhs: &Spectrum) -> Spectrum {
    Spectrum::new(0.0)
  }
}

pub trait SpectrumRhsDiv<S> {
  fn div_with_spectrum(&self, lhs: &Spectrum) -> S;
}

impl<S, R: SpectrumRhsDiv<S>> Div<R, S> for Spectrum {
  fn div(&self, rhs: &R) -> S {
    rhs.div_with_spectrum(self)
  }
}

impl SpectrumRhsDiv<Spectrum> for f32 {
  fn div_with_spectrum(&self, lhs: &Spectrum) -> Spectrum {
    Spectrum::new(0.0)
  }
}

impl SpectrumRhsDiv<Spectrum> for Spectrum {
  fn div_with_spectrum(&self, lhs: &Spectrum) -> Spectrum {
    Spectrum::new(0.0)
  }
}
