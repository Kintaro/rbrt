use film::Film;
use geometry::{ Ray, RayDifferential };

pub struct CameraBase {
    shutter_open:  f32,
    shutter_close: f32,
    film:         ~Film
}

pub trait Camera {
    fn generate_ray(ray: &Ray);
    fn generate_ray_differential(rd: &RayDifferential);
}