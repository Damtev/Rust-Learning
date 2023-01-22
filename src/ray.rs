use crate::color::Color;
use crate::sphere::hit_sphere;
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
        let t = hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, self);
        if t > 0.0 {
            let n = (self.at(t) - Vec3::new(0.0, 0.0, -1.0)).normalized();

            return 0.5 * Color::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
        }

        let unit_direction = self.direction.normalized();
        let t = 0.5 * (unit_direction.y() + 1.0);

        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}
