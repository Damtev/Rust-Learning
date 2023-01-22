use crate::ray::Ray;
use crate::vec3::Point3;

pub fn check_hit_sphere(center: Point3, radius: f64, ray: &Ray) -> bool {
    let oc = ray.origin() - center;

    let a = ray.direction() * ray.direction();
    let b = 2.0 * oc * ray.direction();
    let c = oc * oc - radius * radius;

    let discriminant = b * b - 4.0 * a * c;

    discriminant > 0.0
}
