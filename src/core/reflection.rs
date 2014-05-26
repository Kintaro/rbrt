use diffgeom::DifferentialGeometry;
use geometry::{ Normal, Vector, dot };
use rand::{ TaskRng, Rng };
use sampler::Sample;
use spectrum::Spectrum;

pub struct BsdfSample {
  pub udir: (f32, f32),
  pub ucomponent: f32
}

impl BsdfSample {
  pub fn new(up0: f32, up1: f32, ucomp: f32) -> BsdfSample {
    BsdfSample { udir: (up0, up1), ucomponent: ucomp }
  }

  pub fn from_random(rng: &mut TaskRng) -> BsdfSample {
    BsdfSample { udir: rng.gen::<(f32, f32)>(), ucomponent: rng.gen::<f32>() }
  }

  pub fn from_sample(sample: &Sample, offsets: &BsdfSampleOffsets, n: uint) -> BsdfSample {
    let a = sample.twoD[offsets.dir_offset][2 * n];
    let b = sample.twoD[offsets.dir_offset][2 * n + 1];
    let u = sample.oneD[offsets.component_offset][n];

    BsdfSample { udir: (a, b), ucomponent: u }
  }
}

pub struct BsdfSampleOffsets {
  pub num_samples: uint,
  pub component_offset: uint,
  pub dir_offset: uint,
}

bitflags!(
  flags BxDFType: u32 {
  static NoType      = 0x00000000,
  static Reflection    = 0x00000001,
  static Transmission  = 0x00000010,
  static Diffuse     = 0x00000100,
  static Glossy      = 0x00001000,
  static Specular    = 0x00010000,
  static AllTypes    = Diffuse.bits | Glossy.bits | Specular.bits,
  static AllReflection   = Reflection.bits | AllTypes.bits,
  static AllTransmission = Transmission.bits | AllTypes.bits,
  static All       = AllReflection.bits | AllTransmission.bits
  }
)

pub struct BxDFBase {
  pub bxdf_type: BxDFType
}

pub trait BxDF<'a> {
  fn get_base(&'a self) -> &'a BxDFBase;
  fn get_base_mut(&'a mut self) -> &'a mut BxDFBase;

  fn f(&'a self, wo: &Vector, wi: &Vector) -> Spectrum;
  fn sample_f(&'a self, wo: &Vector, wi: &mut Vector, u1: f32, u2: f32, pdf: &mut f32) -> (Spectrum, f32);
  fn rho(&'a self, wo: Vector, num_samples: uint, samples: &[f32]) -> Spectrum;
  fn rho2(&'a self, num_samples: uint, samples1: &[f32], samples2: &[f32]) -> Spectrum;
  fn pdf(&'a self, wi: &Vector, wo: &Vector) -> f32;

  fn matches_flags(&'a self, flags: BxDFType) -> bool {
    self.get_base().bxdf_type.contains(flags)
  }
}

pub struct Bsdf<'a> {
  pub dg_shading: DifferentialGeometry<'a>,
  pub eta: f32,
  pub nn: Normal,
  pub ng: Normal,
  pub sn: Vector,
  pub tn: Vector,
  pub nbxdfs: uint,
  pub bxdfs: [Option<Box<BxDF<'a>>>, ..8]
}

impl<'a> Bsdf<'a> {
  pub fn add(&'a mut self, bxdf: Box<BxDF<'a>>) {
    self.bxdfs[self.nbxdfs] = Some(bxdf);
    self.nbxdfs += 1;
  }

  pub fn world_to_local(&'a self, v: &Vector) -> Vector {
    Vector::new(dot(*v, self.sn), dot(*v, self.tn), dot(*v, self.nn))
  }

  pub fn local_to_world(&'a self, v: &Vector) -> Vector {
    Vector::new(self.sn.x * v.x + self.tn.x * v.y + self.nn.x * v.z,
      self.sn.y * v.x + self.tn.y * v.y + self.nn.y * v.z,
      self.sn.z * v.x + self.tn.z * v.z + self.nn.z * v.z)
  }

  pub fn sample_f(&'a self, woW: &Vector, wiW: &mut Vector, bsdf_sample: &BsdfSample,
    pdf: &mut f32, flags: &mut BxDFType) -> (Spectrum, BxDFType) {
    let matching_components = 0;

    if matching_components == 0 {
      *pdf = 0.0;

      return (Spectrum::new(0.0), NoType);
    }

    let mut bxdf : Option<&Box<BxDF<'a>>> = None;
    let mut count = 0;
    let n = self.nbxdfs;

    for i in range(0, n) {
      match self.bxdfs[i] {
        Some(ref x) => {
          if x.matches_flags(*flags) && count == 0 {
            count -= 1;
            bxdf = Some(x);
            break;
          } else {
            count -= 1;
          }
        },
        None => (),
      }
    }

    let wo = self.world_to_local(woW);
    let mut wi = Vector::new(0.0, 0.0, 0.0);
    let (mut f, _) = bxdf.unwrap().sample_f(&wo, &mut wi, bsdf_sample.udir.val0(),
      bsdf_sample.udir.val1(), pdf);

    if *pdf == 0.0 {
      return (Spectrum::new(0.0), NoType);
    }

    *wiW = self.local_to_world(&wi);

    if bxdf.unwrap().get_base().bxdf_type & Specular != NoType {
      f = Spectrum::new(0.0);
      if dot(*wiW, self.ng) * dot(*woW, self.ng) > 0.0 {
        *flags = *flags - Transmission;
      } else {
        *flags = *flags - Reflection;
      }

      for i in range(0, self.nbxdfs) {
        match self.bxdfs[i] {
          Some(ref x) => {
            if x.matches_flags(*flags) {
              f = f + x.f(&wo, &wi);
            }
          },
          None => ()
        }
      }
    }

    (f, bxdf.unwrap().get_base().bxdf_type)
  }

  pub fn f(&self, woW: &Vector, wiW: &Vector, flags: BxDFType) -> Spectrum {
    fail!("not implemented");
  }
}
