use film::Film;
use geometry::{ Ray, RayDifferential };

pub struct CameraBase {
    shutter_open:  f32,
    shutter_close: f32,
    film:         ~Film
}

pub trait Camera<'a> {
    fn get_base(&'a self) -> &'a CameraBase;
    fn get_base_mut(&'a mut self) -> &'a mut CameraBase;
    fn generate_ray(&'a self, ray: &Ray);
    fn generate_ray_differential(&'a self, rd: &RayDifferential);
}