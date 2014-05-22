pub struct SamplerBase;

pub trait Sampler {
    fn get_more_samples(&self, sample: &Sample);
    fn maximum_sample_count(&self);
}

pub struct CameraSampleBase {
    image_x: f32,
    image_y: f32,
    lens_u:  f32,
    lens_v:  f32,
    time:    f32
}

pub struct Sample {
    camera_sample:  CameraSampleBase,
    pub n1D:       Vec<uint>,
    pub n2D:       Vec<uint>,
    pub oneD:      ~[~[f32]],
    pub twoD:      ~[~[f32]]
}

impl Sample {
    pub fn new(sampler: &Sampler) -> Sample {
        fail!("")
    }

    pub fn add_1d(&mut self, num: uint) -> uint {
        self.n1D.push(num);
        self.n1D.len() - 1
    }

    pub fn add_2d(&mut self, num: uint) -> uint {
        self.n2D.push(num);
        self.n2D.len() - 1   
    }

    pub fn duplicate(&self, count: uint) -> Sample {
        fail!("Unimplemented method");
    }
}