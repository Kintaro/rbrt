use diffgeom::DifferentialGeometry;
use geometry::{ Normal, Vector };
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
    static Reflection      = 0x00000001,
    static Transmission    = 0x00000010,
    static Diffuse         = 0x00000100,
    static Glossy          = 0x00001000,
    static Specular        = 0x00010000,
    static AllTypes        = Diffuse.bits | Glossy.bits | Specular.bits,
    static AllReflection   = Reflection.bits | AllTypes.bits,
    static AllTransmission = Transmission.bits | AllTypes.bits,
    static All             = AllReflection.bits | AllTransmission.bits
  }
)

pub struct BxDFBase {
    pub bxdf_type: BxDFType
}

pub trait BxDF<'a> {
    fn get_base(&'a self) -> &'a BxDFBase;
    fn get_base_mut(&'a mut self) -> &'a mut BxDFBase;

    fn f(&'a self, wo: &Vector, wi: &Vector) -> Spectrum;
    fn sample_f(&'a self, wo: &Vector, wi: &mut Vector, u1: f32, u2: f32) -> (Spectrum, f32);
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

    pub fn sample_f(&'a self, woW: &Vector, wiW: &mut Vector, bsdf_sample: &BsdfSample, flags: BxDFType) -> (Spectrum, f32, BxDFType) {
        let matching_components = 0;

        if matching_components == 0 {
            fail!("");
        }

        fail!("");
    }
}
