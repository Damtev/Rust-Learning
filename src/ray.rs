use crate::color::Color;
use crate::hit::Hit;
use crate::sphere::hit_sphere;
use crate::vec3::{Point3, Vec3};
use crate::world::World;

pub struct Ray {
    origin: Vec3,
    direction: Vec3
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn origin(&self) -> Vec3 {
        self.origin
    }
    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }

    pub fn color(&self, world: &World, depth: u64) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        if let Some(rec) = world.hit(self, 0.0, f64::INFINITY) {
            let target = rec.p + rec.normal + Vec3::random_in_unit_sphere();
            let ray = Ray::new(rec.p, target - rec.p);

            0.5 * ray.color(world, depth - 1)
        } else {
            let unit_direction = self.direction().normalized();
            let t = 0.5 * (unit_direction.y() + 1.0);

            (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
        }
    }
}
