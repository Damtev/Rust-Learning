use crate::color::Color;
use crate::sphere::check_hit_sphere;
use crate::vec3::{Point3, Vec3};

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

    pub fn color(&self) -> Color {
        if check_hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, self) {
            return Color::new(1.0, 0.0, 0.0);
        }

        let unit_direction = self.direction.normalized();
        let t = 0.5 * (unit_direction.y() + 1.0);

        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}
