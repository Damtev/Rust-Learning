use crate::vec3::Vec3;

pub type Color = Vec3;

impl Color {
    pub fn format(&self, samples_per_pixel: u64) -> String {
        let ir = (256.0 * (self.x() / (samples_per_pixel as f64)).sqrt().clamp(0.0, 0.999)) as u64;
        let ig = (256.0 * (self.y() / (samples_per_pixel as f64)).sqrt().clamp(0.0, 0.999)) as u64;
        let ib = (256.0 * (self.z() / (samples_per_pixel as f64)).sqrt().clamp(0.0, 0.999)) as u64;

        format!("{} {} {}", ir, ig, ib)
    }
}
