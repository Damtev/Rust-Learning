use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3
}

impl Camera {
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
    ) -> Self {
        // Vertical field-of-view in degrees
        let theta = std::f64::consts::PI / 180.0 * vfov;
        let viewport_height = 2.0 * (theta / 2.0).tan();
        let viewport_width = aspect_ratio * viewport_height;

        let cw = (lookfrom - lookat).normalized();
        let cu = vup.cross_product(cw).normalized();
        let cv = cw.cross_product(cu);

        let horizontal = viewport_width * cu;
        let vertical = viewport_height * cv;

        let lower_left_corner = lookfrom - horizontal / 2.0 - vertical / 2.0 - cw;

        Camera {
            origin: lookfrom,
            lower_left_corner,
            horizontal,
            vertical
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin)
    }
}
