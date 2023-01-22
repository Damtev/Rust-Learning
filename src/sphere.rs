use crate::ray::Ray;
use crate::vec3::Point3;

pub fn hit_sphere(center: Point3, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.origin() - center;

    let a = ray.direction() * ray.direction();
    let b = 2.0 * oc * ray.direction();
    let c = oc * oc - radius * radius;

    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0 * a)
    }
}
