use rand::Rng;
use crate::color::Color;
use crate::hit::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub trait Scatter : Send + Sync {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    albedo: Color
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian {
            albedo
        }
    }
}

impl Scatter for Lambertian {
    fn scatter(&self, _ray: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = rec.normal + Vec3::random_in_unit_sphere().normalized();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        let scattered = Ray::new(rec.p, scatter_direction);

        Some((self.albedo, scattered))
    }
}

pub struct Metal {
    albedo: Color,
    fuzzing: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzzing: f64,) -> Metal {
        Metal {
            albedo,
            fuzzing,
        }
    }
}

impl Scatter for Metal {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = ray.direction().reflect(rec.normal).normalized();
        let scattered = Ray::new(rec.p, reflected + self.fuzzing * Vec3::random_in_unit_sphere());

        if scattered.direction() * rec.normal > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    index_of_refraction: f64
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Dielectric {
        Dielectric {
            index_of_refraction
        }
    }

    pub fn reflectance(cosine: f64, index_of_refraction: f64) -> f64 {
        // Use Schlick's approximation for reflectance
        let r0 = ((1.0 - index_of_refraction) / (1.0 + index_of_refraction)).powi(2);

        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Scatter for Dielectric {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let refraction_ratio = if rec.front_face {
            1.0 / self.index_of_refraction
        } else {
            self.index_of_refraction
        };

        let unit_direction = ray.direction().normalized();
        let cos_theta = (-1.0 * unit_direction * rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let mut random = rand::thread_rng();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let will_reflect = random.gen::<f64>() < Self::reflectance(cos_theta, refraction_ratio);

        let direction = if cannot_refract || will_reflect {
            unit_direction.reflect(rec.normal)
        } else {
            unit_direction.refract(rec.normal, refraction_ratio)
        };

        let scattered = Ray::new(rec.p, direction);

        Some((Color::new(1.0, 1.0, 1.0), scattered))
    }
}
