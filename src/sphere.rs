use crate::ray::Ray;
use crate::vec3::Point3;

pub fn hit_sphere(center: Point3, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.origin() - center;

    let a = ray.direction().length().powi(2);
    let half_b = oc * ray.direction();
    let c = oc.length().powi(2) - radius * radius;

    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / a
    }
}
