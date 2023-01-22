use crate::vec3::Vec3;

pub type Color = Vec3;

impl Color {
    pub fn format(self) -> String {
        format!("{} {} {}",
                (255.999 * self.x()) as u64,
                (255.999 * self.y()) as u64,
                (255.999 * self.z()) as u64
        )
    }
}
