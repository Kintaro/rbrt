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

pub enum BxDFType {
    Reflection    = 1 << 0,
    Transmission  = 1 << 1,
    Diffuse       = 1 << 2,
    Glossy        = 1 << 3,
    Specular      = 1 << 4,
    AllTypes      = 0x1c,
    AllReflection = 0x1d,
    All           = 0x1f,
}

pub struct BxDFBase {
    pub bxdf_type: BxDFType
}

pub trait BxDF {
    fn get_base(&self) -> BxDFBase;
    fn f(&self, wo: &Vector, wi: &Vector) -> Spectrum;
    fn sample_f(&self, wo: &Vector, wi: &mut Vector, u1: f32, u2: f32) -> (Spectrum, f32);
    fn rho(&self, wo: Vector, num_samples: uint, samples: &[f32]) -> Spectrum;
    fn rho2(&self, num_samples: uint, samples1: &[f32], samples2: &[f32]) -> Spectrum;
    fn pdf(&self, wi: &Vector, wo: &Vector) -> f32;

    fn matches_flags(&self, flags: BxDFType) -> bool {
        self.get_base().bxdf_type as uint & flags as uint == self.get_base().bxdf_type as uint
    }
}

pub struct Bsdf {
    pub nn: Normal,
    pub ng: Normal,
    pub sn: Vector,
    pub tn: Vector,
    pub nbxdfs: uint,
    pub bxdfs: [Option<~BxDF>, ..8]
}

impl Bsdf {
    pub fn add(&mut self, bxdf: ~BxDF) {
        self.bxdfs[self.nbxdfs] = Some(bxdf);
        self.nbxdfs += 1;
    }

    pub fn sample_f(woW: &Vector, wiW: &mut Vector, bsdf_sample: &BsdfSample, flags: BxDFType) -> (Spectrum, f32, BxDFType) {
        let matching_components = 0;

        if matching_components == 0 {
            fail!("");
        }

        fail!("");
    }
}